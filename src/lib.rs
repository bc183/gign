use clap::Parser;
use reqwest;
use std::{fs::File, io::Write, path::Path};

/// A command line utility to generate gitignore files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Languages to generate gitignore for. Should be separated by commas.
    #[arg(short, long)]
    lang: Option<String>,

    /// The path to generate gitignore. Default is the current working directory.
    #[arg(short, long)]
    path: Option<String>,

    /// The available templates to generate gitignore.
    #[arg(short, long, default_value_t = false)]
    avail: bool,
}

pub fn run(args: &Args) -> Result<(), &str> {
    // check if list is the commandtrue
    if args.avail {
        // return all the possible gitignore list
        let avl_langs = match get_list_of_available_langs() {
            Ok(l) => l,
            Err(e) => return Err(e),
        };
        for (idx, lang) in avl_langs.iter().enumerate() {
            println!("{}. {}", idx + 1, lang);
        }
        return Ok(());
    }

    let languages: Vec<String> = args
        .lang
        .clone()
        .unwrap_or(String::from("rust"))
        .split_whitespace()
        .map(|item| item.to_string())
        .collect();

    let path = match args.path.clone() {
        Some(p) => p,
        None => String::from("./"),
    };
    let file_path = Path::new(&path);
    // check if the path exists
    if !file_path.exists() {
        return Err("File path does not exist");
    }

    let gitignore = match get_gitignore(&languages) {
        Ok(d) => d,
        Err(e) => {
            return Err(e);
        }
    };
    if let Err(e) = create_gitignore(gitignore, file_path) {
        return Err(e);
    }
    Ok(())
}

fn get_list_of_available_langs() -> Result<Vec<String>, &'static str> {
    let url = String::from("https://www.toptal.com/developers/gitignore/api/list");
    let data = match reqwest::blocking::get(url) {
        Ok(d) => d,
        Err(_) => return Err("Couldn't get supported langs. Try again later."),
    };
    let text = match data.text() {
        Ok(d) => d,
        Err(_) => return Err("Couldn't get supported langs. Try again later."),
    };
    let mut result = vec![];
    for lang in text.split(",") {
        result.push(lang.to_owned());
    }
    Ok(result)
}

pub fn get_gitignore(langs: &Vec<String>) -> Result<String, &'static str> {
    let mut url = String::from("https://www.toptal.com/developers/gitignore/api/");
    url.push_str(&langs.join(","));
    let data = match reqwest::blocking::get(url) {
        Ok(d) => d,
        Err(_) => return Err("Couldn't create gitignore. Try again later."),
    };
    match data.text() {
        Ok(d) => Ok(d),
        Err(_) => Err("Couldn't create gitignore. Try again later."),
    }
}

pub fn create_gitignore(content: String, path: &Path) -> Result<(), &'static str> {
    // create the gitignore file
    let path = path.join(".gitignore");
    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(_) => return Err("Couldn't create gitignore"),
    };

    // write to file
    match file.write(content.as_bytes()) {
        Ok(b) => b,
        Err(_) => return Err("Couldn't create gitignore"),
    };

    Ok(())
}
