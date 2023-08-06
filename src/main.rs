use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use clap::Parser;
use serde::Deserialize;
use url::Url;

/// A cli tool for viewing the documentation for a language.
/// It uses the excellent learnxinyminutes.com website by Adam Bard.
/// The src code examples are saved in /var/tmp/x-in-y.
#[derive(Parser, Default)]
#[command(about = "View documentation in code form", long_about = None)]
struct Cli {
    /// Language to find documentation for, e.g. rust, python, javascript.
    #[arg(help = "Language to find documentation for", required_unless_present = "show_languages")]
    language: Option<String>,

    /// You can optionally specify a viewer to view the code in.
    /// This is useful with e.g. bat. If you specify with arg, you get syntax highlighting.
    /// If you instead pipe, the code will not be highlighted.
    #[arg(short, long, help = "Specify the viewer to use, e.g. less, bat, cat", long_help)]
    viewer: Option<String>,
    /// List all available languages to view.
    #[arg(default_value = "false", short, long, help = "List all languages", long_help)]
    show_languages: bool,
}


#[derive(Debug, Deserialize)]
struct LinkInfo {
    name: String,
    source_code_link: String,
    contributor_text: String,
    contributor_link: String,
}

const URL_PREFIX: &str = "https://learnxinyminutes.com/";


fn get_infos() -> Result<Vec<LinkInfo>, Box<serde_json::Error>> {
    let text = include_str!("../data.json");
    let infos: Vec<LinkInfo> = serde_json::from_str(text)?;
    Ok(infos)
}

fn init(cli: Cli) -> (String, Option<String>, PathBuf) {
    let language: String = cli.language.unwrap().to_lowercase();
    let viewer = cli.viewer;

    let folder = PathBuf::from("/var/tmp/x-in-y");
    (language, viewer, folder)
}

fn display_available_languages(info_vec: Vec<LinkInfo>){
    let lang_string = show_available_langs(info_vec);
    println!("Available languages:\n{}", lang_string);
}

fn display_documentation(cli: Cli, info_vec: Vec<LinkInfo>)-> Result<(), Box<dyn std::error::Error>>{
        let (language, viewer, folder) = init(cli);
        let info = info_vec.into_iter()
            .find(|info| *info.name.to_lowercase() == language.to_lowercase())
            .ok_or("Could not find language")?;

        let url = Url::parse(&format!("{}{}", URL_PREFIX, &info.source_code_link))?;

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
            Some(viewer) if cfg!(target_os = "linux") => { run_process(viewer, file_path, credit_path) }
            _ => { print_stdout(file_path, credit).expect("Could not print to stdout."); }
        }
        Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let info_vec = get_infos()?;
    if cli.show_languages {
        display_available_languages(info_vec);
    } else {
        display_documentation(cli, info_vec)?;
    }
    Ok(())
}

fn show_available_langs(info_vec: Vec<LinkInfo>) -> String {
    info_vec.into_iter().map(|info| info.name).collect::<Vec<String>>().join("\n")
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
