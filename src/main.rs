use clap::Parser;
use serde::Deserialize;
use std::fs::{self, create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use url::Url;

/// The src code examples are saved in /var/tmp/x-in-y
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The language to find documentation for
    #[arg(short, long)]
    language: String,

    /// The default way to view content is less
    #[arg(short, long)]
    viewer: Option<String>,
}
#[derive(Debug, Deserialize)]
struct LinkInfo {
    name: String,
    language_link: String,
    source_code_link: String,
    contributor_text: String,
    contributor_link: String,
}

const URL_PREFIX: &str = "https://learnxinyminutes.com/";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let language: String = args.language.to_lowercase();
    let viewer = args.viewer.unwrap_or("less".to_string());

    let text = include_str!("../data.json");
    let infos: Vec<LinkInfo> = serde_json::from_str(text)?;

    let folder = PathBuf::from("/var/tmp/x-in-y");

    // if folder.exists() {
    //     fs::remove_dir_all(&folder).unwrap();
    // }
    let info = infos
        .into_iter()
        .find(|info| *info.name.to_lowercase() == language.to_lowercase())
        .ok_or("Could not find language")?;

    let url = Url::parse(&format!("{}{}", URL_PREFIX, &info.source_code_link)).unwrap();

    let path_segments = url.path_segments().ok_or("cannot be base")?;
    let file_name = path_segments
        .last()
        .expect("should be able to find name of file from url");
    let file_path = folder.join(file_name);
    let credit_path = file_path.with_extension("txt");

    if !file_path.exists() {
        create_dir_all(folder)?;
        let res = reqwest::blocking::get(url)?;
        let contents = res.text()?;
        let mut file = File::create(&file_path).expect("could not create file.");
        file.write_all(contents.as_bytes())
            .expect("could not write to file");
    }

    if !credit_path.exists() {
        let credit = format!(
            "
==============================================================================
credit
==============================================================================
{}

{}",
            info.contributor_text, info.contributor_link
        );
        let mut file = File::create(&credit_path).expect("could not create file.");
        file.write_all(credit.as_bytes())
            .expect("could not write to file");
    }

    std::process::Command::new(viewer)
        .args([file_path, credit_path])
        .spawn()
        .expect("command failed")
        .wait()
        .expect("wait failed");
    Ok(())
}
