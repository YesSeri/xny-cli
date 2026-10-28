use std::error::Error;
use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};

use clap::{CommandFactory, Parser};
use serde::{Deserialize, Serialize};
use url::Url;

/// A cli tool for viewing the documentation for a language.
/// It uses the excellent learnxinyminutes.com website by Adam Bard.
/// Downloaded examples and catalogue are stored in the platform cache directory.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Language to find documentation for, e.g. rust, python, javascript.
    language: Option<String>,

    /// Specify the viewer to use, e.g. less, bat, cat.
    #[clap(short, long)]
    #[arg(conflicts_with_all = ["show_languages", "clear_cache"])]
    viewer: Option<String>,

    /// List all available languages to view.
    #[clap(short, long, action)]
    #[arg(conflicts_with_all = ["clear_cache", "language"])]
    show_languages: bool,

    /// Clears the local cache of docs
    #[clap(short, long, action)]
    #[arg(conflicts_with_all = ["language", "viewer", "show_languages", "update_catalog"])]
    clear_cache: bool,

    /// Refresh the catalogue from the Learn X in Y Minutes repository.
    #[clap(long, action)]
    #[arg(conflicts_with = "clear_cache")]
    update_catalog: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct LinkInfo {
    name: String,
    filename: String,
    contributor_text: String,
    contributor_link: String,
}

#[derive(Debug, Deserialize)]
struct FrontMatter {
    name: String,
    filename: Option<String>,
    contributors: Option<Vec<Vec<String>>>,
}

const URL_PREFIX: &str = "https://learnxinyminutes.com/";
const MAX_ARCHIVE_BYTES: usize = 32 * 1024 * 1024;
const MAX_ENTRY_BYTES: u64 = 2 * 1024 * 1024;
const MAX_GUIDE_BYTES: usize = 4 * 1024 * 1024;
const DOCS_ARCHIVE_URL: &str =
    "https://github.com/adambard/learnxinyminutes-docs/archive/refs/heads/master.zip";

fn get_infos(refresh: bool) -> Result<Vec<LinkInfo>, Box<dyn Error>> {
    let cache = cache_path()?;
    let catalog = cache.join("catalog.json");
    get_infos_from(&catalog, refresh, fetch_catalog)
}

fn get_infos_from<F>(
    catalog: &Path,
    refresh: bool,
    fetch: F,
) -> Result<Vec<LinkInfo>, Box<dyn Error>>
where
    F: FnOnce() -> Result<Vec<LinkInfo>, Box<dyn Error>>,
{
    let cached: Option<Vec<LinkInfo>> = std::fs::read_to_string(catalog)
        .ok()
        .and_then(|contents| serde_json::from_str(&contents).ok());
    if !refresh {
        if let Some(infos) = cached.as_ref() {
            return Ok(infos.clone());
        } else if catalog.exists() {
            eprintln!("Warning: cached catalogue is invalid; refreshing it.");
        }
    }

    let infos = match fetch() {
        Ok(infos) => infos,
        Err(error) if cached.is_some() => {
            eprintln!("Warning: catalogue refresh failed ({error}); using cached catalogue.");
            return Ok(cached.expect("checked above"));
        }
        Err(error) => return Err(error),
    };
    if let Some(parent) = catalog.parent() {
        create_dir_all(parent)?;
    }
    let serialized = serde_json::to_vec_pretty(&infos)?;
    replace_file(catalog, &serialized)?;
    Ok(infos)
}

fn fetch_catalog() -> Result<Vec<LinkInfo>, Box<dyn Error>> {
    let mut response = minreq::get(DOCS_ARCHIVE_URL).with_timeout(30).send_lazy()?;
    if !(200..300).contains(&response.status_code) {
        return Err(format!("catalogue download returned HTTP {}", response.status_code).into());
    }
    let archive = read_limited(
        &mut response,
        MAX_ARCHIVE_BYTES,
        "upstream catalogue archive",
    )?;
    parse_catalog_archive(archive)
}

fn parse_catalog_archive(archive: Vec<u8>) -> Result<Vec<LinkInfo>, Box<dyn Error>> {
    let mut zip = zip::ZipArchive::new(Cursor::new(archive))?;
    let mut infos = Vec::new();

    for index in 0..zip.len() {
        let mut entry = zip.by_index(index)?;
        let path = entry.name()?.into_owned();
        let Some(relative_path) = path.split_once('/').map(|(_, path)| path) else {
            continue;
        };
        if relative_path.contains('/') || !relative_path.ends_with(".md") {
            continue;
        }
        if entry.size() > MAX_ENTRY_BYTES {
            eprintln!("Skipping {relative_path}: file exceeds the 2 MiB limit");
            continue;
        }

        let markdown_bytes = read_limited(&mut entry, MAX_ENTRY_BYTES as usize, "catalogue entry")?;
        let markdown = String::from_utf8_lossy(&markdown_bytes);
        let Some(front_matter) = parse_front_matter(&markdown) else {
            eprintln!("Skipping {relative_path}: missing frontmatter");
            continue;
        };
        let metadata: FrontMatter = match serde_yaml_ng::from_str(front_matter) {
            Ok(metadata) => metadata,
            Err(error) => {
                eprintln!("Skipping {relative_path}: invalid frontmatter ({error})");
                continue;
            }
        };
        let Some(filename) = metadata.filename.filter(|filename| !filename.is_empty()) else {
            eprintln!("Skipping {relative_path}: filename is missing");
            continue;
        };
        if !is_safe_filename(&filename) {
            eprintln!("Skipping {relative_path}: invalid output filename");
            continue;
        }

        let contributors = metadata.contributors.unwrap_or_default();
        let credit = contributors
            .first()
            .map(|person| {
                let name = person
                    .first()
                    .map(String::as_str)
                    .unwrap_or("Unknown contributor");
                if contributors.len() > 1 {
                    format!(
                        "Originally contributed by {name}, and updated by {} contributor(s).",
                        contributors.len() - 1
                    )
                } else {
                    format!("Originally contributed by {name}, and updated by 0 contributor(s).")
                }
            })
            .unwrap_or_else(|| "No contributor information available.".to_owned());

        infos.push(LinkInfo {
            name: metadata.name,
            filename,
            contributor_text: credit,
            contributor_link: format!(
                "https://github.com/adambard/learnxinyminutes-docs/blame/master/{relative_path}"
            ),
        });
    }

    sort_catalogue(&mut infos);
    if infos.is_empty() {
        return Err("No guides found in the upstream docs archive".into());
    }
    Ok(infos)
}

fn read_limited(
    reader: &mut impl Read,
    limit: usize,
    label: &str,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut bytes = Vec::with_capacity(limit.min(64 * 1024));
    reader.take((limit + 1) as u64).read_to_end(&mut bytes)?;
    if bytes.len() > limit {
        return Err(format!("{label} exceeds the {} byte limit", limit).into());
    }
    Ok(bytes)
}

fn is_safe_filename(filename: &str) -> bool {
    Path::new(filename)
        .file_name()
        .and_then(|name| name.to_str())
        == Some(filename)
}

fn sort_catalogue(infos: &mut [LinkInfo]) {
    infos.sort_by(|left, right| {
        left.name
            .chars()
            .flat_map(char::to_lowercase)
            .cmp(right.name.chars().flat_map(char::to_lowercase))
            .then_with(|| left.name.cmp(&right.name))
    });
}

fn same_language(left: &str, right: &str) -> bool {
    left.chars()
        .flat_map(char::to_lowercase)
        .eq(right.chars().flat_map(char::to_lowercase))
}

fn parse_front_matter(markdown: &str) -> Option<&str> {
    let rest = markdown.strip_prefix("---")?;
    let rest = rest
        .strip_prefix('\n')
        .or_else(|| rest.strip_prefix("\r\n"))?;
    let end = rest.find("\n---")?;
    Some(&rest[..end])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_frontmatter() {
        assert_eq!(
            parse_front_matter("---\nname: Rust\n---\nbody"),
            Some("name: Rust")
        );
        assert_eq!(parse_front_matter("no metadata"), None);
    }

    #[test]
    fn sorts_catalogue_names_case_insensitively() {
        let mut infos = ["rust", "Ada", "C"]
            .map(|name| LinkInfo {
                name: name.to_string(),
                filename: String::new(),
                contributor_text: String::new(),
                contributor_link: String::new(),
            })
            .to_vec();
        sort_catalogue(&mut infos);
        assert_eq!(show_available_langs(infos), "Ada\nC\nrust");
    }

    #[test]
    fn refreshes_invalid_cached_catalogue() {
        let path = std::env::temp_dir().join(format!("xny-catalog-{}.json", std::process::id()));
        std::fs::write(&path, "not json").unwrap();
        let result = get_infos_from(&path, false, || {
            Ok(vec![LinkInfo {
                name: "Rust".into(),
                filename: "rust.html".into(),
                contributor_text: String::new(),
                contributor_link: String::new(),
            }])
        })
        .unwrap();
        assert_eq!(result[0].name, "Rust");
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn language_matching_is_case_insensitive_for_unicode() {
        assert!(same_language("Ångström", "ångström"));
        assert!(!same_language("Ångström", "angstrom"));
    }

    #[test]
    fn sorts_unicode_names_and_breaks_case_ties_deterministically() {
        let mut infos = ["ångström", "Zebra", "Ångström"].map(|name| LinkInfo {
            name: name.to_string(),
            filename: String::new(),
            contributor_text: String::new(),
            contributor_link: String::new(),
        });
        sort_catalogue(&mut infos);
        assert_eq!(
            infos.map(|info| info.name),
            ["Zebra", "Ångström", "ångström"]
        );
    }

    #[test]
    fn accepts_cli_option_combinations_and_rejects_conflicts() {
        assert!(Cli::try_parse_from(["xny", "--update-catalog", "rust"]).is_ok());
        assert!(Cli::try_parse_from(["xny", "--update-catalog", "--show-languages"]).is_ok());
        assert!(Cli::try_parse_from(["xny", "--show-languages", "rust"]).is_err());
        assert!(Cli::try_parse_from(["xny", "--show-languages", "--viewer", "cat"]).is_err());
        assert!(Cli::try_parse_from(["xny", "--clear-cache", "rust"]).is_err());
    }

    #[test]
    fn validates_output_filenames() {
        assert!(is_safe_filename("rust.html"));
        assert!(is_safe_filename("Learn Rust.txt"));
        assert!(!is_safe_filename("../rust.html"));
        assert!(!is_safe_filename("dir/rust.html"));
    }

    #[test]
    fn reads_data_at_limit_and_rejects_data_over_limit() {
        let mut exact = Cursor::new(b"1234");
        assert_eq!(read_limited(&mut exact, 4, "test input").unwrap(), b"1234");
        let mut oversized = Cursor::new(b"12345");
        assert!(read_limited(&mut oversized, 4, "test input").is_err());
    }

    #[test]
    fn parses_crlf_frontmatter_and_rejects_unclosed_frontmatter() {
        assert_eq!(
            parse_front_matter("---\r\nname: Rust\r\n---\r\nbody"),
            Some("name: Rust\r")
        );
        assert_eq!(parse_front_matter("---\nname: Rust\nbody"), None);
    }

    #[test]
    fn valid_cache_skips_fetch_and_failed_refresh_uses_valid_cache() {
        let path = std::env::temp_dir().join(format!("xny-cache-test-{}.json", std::process::id()));
        let cached = vec![LinkInfo {
            name: "Rust".into(),
            filename: "rust.html".into(),
            contributor_text: String::new(),
            contributor_link: String::new(),
        }];
        std::fs::write(&path, serde_json::to_vec(&cached).unwrap()).unwrap();

        let loaded =
            get_infos_from(&path, false, || panic!("valid cache should skip fetch")).unwrap();
        assert_eq!(loaded[0].name, "Rust");

        let fallback = get_infos_from(&path, true, || Err("network unavailable".into())).unwrap();
        assert_eq!(fallback[0].name, "Rust");
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn invalid_cache_does_not_hide_refresh_failure() {
        let path = std::env::temp_dir().join(format!(
            "xny-invalid-cache-test-{}.json",
            std::process::id()
        ));
        std::fs::write(&path, "not json").unwrap();
        let error = get_infos_from(&path, false, || Err("network unavailable".into())).unwrap_err();
        assert!(error.to_string().contains("network unavailable"));
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn parses_archive_guides_and_skips_nested_or_unsafe_entries() {
        let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
        let options = zip::write::SimpleFileOptions::default();
        writer.start_file("docs/rust.md", options).unwrap();
        writer
            .write_all(b"---\nname: Rust\nfilename: rust.html\n---\nGuide")
            .unwrap();
        writer
            .start_file("docs/nested/ignored.md", options)
            .unwrap();
        writer
            .write_all(b"---\nname: Ignore\nfilename: ignore.html\n---\nGuide")
            .unwrap();
        writer.start_file("docs/traversal.md", options).unwrap();
        writer
            .write_all(b"---\nname: Unsafe\nfilename: ../unsafe.html\n---\nGuide")
            .unwrap();
        let archive = writer.finish().unwrap().into_inner();

        let infos = parse_catalog_archive(archive).unwrap();
        assert_eq!(infos.len(), 1);
        assert_eq!(infos[0].name, "Rust");
        assert_eq!(infos[0].filename, "rust.html");
    }
}

fn display_available_languages(info_vec: Vec<LinkInfo>) {
    let lang_string = show_available_langs(info_vec);
    println!("Available languages:\n{}", lang_string);
}

fn display_documentation(
    language: &str,
    viewer: Option<String>,
    info_vec: Vec<LinkInfo>,
) -> Result<(), Box<dyn Error>> {
    let folder = cache_path()?;
    let info = info_vec
        .into_iter()
        .find(|info| same_language(&info.name, language))
        .ok_or("Could not find language")?;

    let base_url = Url::parse(URL_PREFIX)?;
    let url = base_url.join(&format!("files/{}", info.filename))?;

    let path_segments = url.path_segments().ok_or("invalid path segments")?;
    let file_name = path_segments.last().ok_or("invalid path segments")?;
    let file_path = folder.join(file_name);
    let credit_path = file_path.with_extension("txt");

    if !file_path.exists() {
        create_dir_all(folder)?;
        let mut response = minreq::get(url.as_str()).with_timeout(30).send_lazy()?;
        if !(200..300).contains(&response.status_code) {
            return Err(format!("guide download returned HTTP {}", response.status_code).into());
        }
        let contents = read_limited(&mut response, MAX_GUIDE_BYTES, "guide download")?;
        replace_file(&file_path, &contents)?;
    }

    let credit = format!(
        "Cli written by Henrik Zenkert, henrik.zenkert@gmail.com\nCode: {}\nFull list:{}",
        info.contributor_text, info.contributor_link
    );

    replace_file(&credit_path, credit.as_bytes())?;

    match viewer {
        Some(viewer) => run_process(viewer, file_path, credit_path)?,
        _ => {
            print_stdout(file_path, credit).expect("Could not print to stdout.");
        }
    }
    Ok(())
}

fn clear_cache() -> Result<(), Box<dyn Error>> {
    let cache = cache_path()?;
    if !cache.is_dir() {
        println!("Nothing to clear");
        return Ok(());
    }

    remove_dir_all(cache)?;
    println!("Successfully cleared cache");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    if cli.clear_cache {
        return Ok(clear_cache()?);
    }

    let info_vec = get_infos(cli.update_catalog)?;

    if cli.show_languages {
        display_available_languages(info_vec);
    } else if let Some(language) = cli.language {
        display_documentation(&language, cli.viewer, info_vec)?;
    } else if !cli.update_catalog {
        Cli::command()
            .error(
                clap::error::ErrorKind::MissingRequiredArgument,
                "provide a language, --show-languages, --update-catalog, or --clear-cache",
            )
            .exit();
    }

    Ok(())
}

fn show_available_langs(info_vec: Vec<LinkInfo>) -> String {
    info_vec
        .into_iter()
        .map(|info| info.name)
        .collect::<Vec<String>>()
        .join("\n")
}

fn run_process(
    viewer: String,
    file_path: PathBuf,
    credit_path: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let status = std::process::Command::new(viewer)
        .args([file_path, credit_path])
        .spawn()
        .map_err(|error| format!("could not start viewer: {error}"))?
        .wait()
        .map_err(|error| format!("could not wait for viewer: {error}"))?;
    if !status.success() {
        return Err(format!("viewer exited with {status}").into());
    }
    Ok(())
}

fn cache_path() -> Result<PathBuf, Box<dyn Error>> {
    Ok(dirs::cache_dir()
        .ok_or("could not determine platform cache directory")?
        .join("xny"))
}

fn replace_file(path: &Path, contents: &[u8]) -> Result<(), Box<dyn Error>> {
    let temporary = path.with_extension("tmp");
    File::create(&temporary)?.write_all(contents)?;
    #[cfg(windows)]
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    std::fs::rename(temporary, path)?;
    Ok(())
}

fn print_stdout(file_path: PathBuf, credit: String) -> std::io::Result<()> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{contents}");
    println!("{credit}");
    Ok(())
}
