use std:: collections:: HashMap;
use std:: fs:: { File, create_dir_all, self };
use std:: path:: PathBuf;
use url:: Url;
use clap:: Parser;



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


fn main() -> Result < (), Box < dyn std:: error:: Error >> {
    let args = Args:: parse();
    let language: String = args.language.to_lowercase();
    let viewer = args.viewer.unwrap_or("less".to_string());

    let text = include_str!("../node/myData.txt");
    let split: Vec <& str > = text.trim().split("\n\n").collect();
    let mut info_map = HashMap:: new();

    for s in split {
        let(name, link) = s.split_once("\n").unwrap();
        info_map.insert(name.to_lowercase(), link);
    }

    let folder = PathBuf:: from("/var/tmp/x-in-y");
    let url = Url:: parse(info_map.get(& language).expect("the language doesn't in hashmap exist")).unwrap();
    let path_segments = url.path_segments().ok_or_else(|| "cannot be base") ?;
    let file_name = path_segments.last().expect("should be able to find name of file from url");
    let file_path = folder.join(file_name);

    if !file_path.exists() {
        create_dir_all(folder.clone()) ?;
        println!("Fetching {:?}...", url);
        let mut res = reqwest:: blocking:: get(url.as_str()) ?;
        let mut file = File:: create(& file_path).expect("could not create file.");
        res.copy_to(& mut file) ?;
    }

    std:: process:: Command:: new (viewer)
        .arg(file_path)
        .spawn()
        .expect("command failed")
        .wait()
        .expect("wait failed");
    Ok(())
}
