use std::env::{Args, args};
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
#[command(about = "Does awesome things", long_about = None)]
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


fn get_infos() -> Result<Vec<LinkInfo>, Box<dyn std::error::Error>> {
    let text = include_str!("../data.json");
    let infos: Vec<LinkInfo> = serde_json::from_str(text).unwrap();
    Ok(infos)
}

fn init(cli: Cli) -> (String, Option<String>, PathBuf) {
    let language: String = cli.language.unwrap().to_lowercase();
    let viewer = cli.viewer;

    let folder = PathBuf::from("/var/tmp/x-in-y");
    (language, viewer, folder)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let info_vec = get_infos()?;
    if cli.show_languages {
        let lang_string = show_available_langs(info_vec);
        println!("Available languages:\n{}", lang_string);
    } else {
        let (language, viewer, folder) = init(cli);
        let info = info_vec.into_iter()
            .find(|info| *info.name.to_lowercase() == language.to_lowercase())
            .ok_or("Could not find language").unwrap();

        println!("lang: {:?}", language);
        println!("view: {:?}", viewer);
        println!("folder: {:?}", folder);
        let url = Url::parse(&format!("{}{}", URL_PREFIX, &info.source_code_link)).unwrap();

        let path_segments = url.path_segments().ok_or("invalid path segments").unwrap();
        let file_name = path_segments.last().ok_or("invalid path segments").unwrap();
        let file_path = folder.join(file_name);
        let credit_path = file_path.with_extension("txt");

        if !file_path.exists() {
            create_dir_all(folder).unwrap();
            let mut file = File::create(&file_path).unwrap();
            file.write_all(minreq::get(url).send().unwrap().as_str().unwrap().as_bytes()).unwrap();
        }

        let credit = format!(
            "Cli written by Henrik Zenkert\nCode: {}\nFull list:{}",
            info.contributor_text, info.contributor_link
        );

        if !credit_path.exists() {
            let mut file = File::create(&credit_path).unwrap();
            file.write_all(credit.as_bytes()).unwrap();
        }

        match viewer {
            Some(viewer) if cfg!(target_os = "linux") => { run_process(viewer, file_path, credit_path) }
            _ => { print_stdout(file_path, credit).expect("Could not print to stdout."); }
        }
    }
    Ok(())
}

fn show_available_langs(info_vec: Vec<LinkInfo>) -> String {
    let mut acc = String::new();
    for info in info_vec {
        acc.push_str(&format!("{}\n", info.name));
    }
    acc
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
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
    println!("{}", credit);
    Ok(())
}
