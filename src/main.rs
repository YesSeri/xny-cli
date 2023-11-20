use std::error::Error;
use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use clap::Parser;
use serde::Deserialize;
use url::Url;

/// A cli tool for viewing the documentation for a language.
/// It uses the excellent learnxinyminutes.com website by Adam Bard.
/// The src code examples are saved in /var/tmp/x-in-y.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Language to find documentation for, e.g. rust, python, javascript.
    language: Option<String>,

    /// Specify the viewer to use, e.g. less, bat, cat.
    #[clap(short, long)]
    viewer: Option<String>,

    /// List all available languages to view.
    #[clap(short, long, action)]
    show_languages: bool,

    /// Clears the local cache of docs
    #[clap(short, long, action)]
    clear_cache: bool,
}

#[derive(Debug, Deserialize)]
struct LinkInfo {
    name: String,
    source_code_link: String,
    contributor_text: String,
    contributor_link: String,
}

const URL_PREFIX: &str = "https://learnxinyminutes.com/";
const CACHE_PATH: &str = "/var/tmp/x-in-y";

fn get_infos() -> Result<Vec<LinkInfo>, Box<serde_json::Error>> {
    let text = include_str!("../data.json");
    let infos: Vec<LinkInfo> = serde_json::from_str(text)?;
    Ok(infos)
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
    let folder = PathBuf::from(CACHE_PATH);
    let info = info_vec
        .into_iter()
        .find(|info| *info.name.to_lowercase() == language.to_lowercase())
        .ok_or("Could not find language")?;

    let base_url = Url::parse(URL_PREFIX)?;
    let url = base_url.join(&info.source_code_link)?;

    let path_segments = url.path_segments().ok_or("invalid path segments")?;
    let file_name = path_segments.last().ok_or("invalid path segments")?;
    let file_path = folder.join(file_name);
    let credit_path = file_path.with_extension("txt");

    if !file_path.exists() {
        create_dir_all(folder)?;
        let mut file = File::create(&file_path)?;
        file.write_all(minreq::get(url).send()?.as_str()?.as_bytes())?;
    }

    let credit = format!(
        "Cli written by Henrik Zenkert, henrik.zenkert@gmail.com\nCode: {}\nFull list:{}",
        info.contributor_text, info.contributor_link
    );

    if !credit_path.exists() {
        let mut file = File::create(&credit_path)?;
        file.write_all(credit.as_bytes())?;
    }

    match viewer {
        Some(viewer) if cfg!(target_os = "linux") => run_process(viewer, file_path, credit_path),
        _ => {
            print_stdout(file_path, credit).expect("Could not print to stdout.");
        }
    }
    Ok(())
}

fn clear_cache() -> Result<(), Box<std::io::Error>> {
    if !Path::new(CACHE_PATH).is_dir() {
        println!("Nothing to clear");
        return Ok(());
    }

    remove_dir_all(CACHE_PATH)?;
    println!("Successfully cleared cache");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    if cli.clear_cache {
        return Ok(clear_cache()?);
    }

    let info_vec = get_infos()?;

    if cli.show_languages {
        display_available_languages(info_vec);
    } else if let Some(language) = cli.language {
        display_documentation(&language, cli.viewer, info_vec)?;
    } else {
        println!(
            r"error: The following required arguments were not provided:
    <LANGUAGE>
USAGE:
    
    xny [OPTIONS] LANGUAGE 
    xny [OPTIONS] --help
    xny [OPTIONS] --version"
        );
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

fn run_process(viewer: String, file_path: PathBuf, credit_path: PathBuf) {
    std::process::Command::new(viewer)
        .args([file_path, credit_path])
        .spawn()
        .expect("command failed")
        .wait()
        .expect("wait failed");
}

fn print_stdout(file_path: PathBuf, credit: String) -> std::io::Result<()> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{contents}");
    println!("{credit}");
    Ok(())
}
