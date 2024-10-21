use clap::Parser;
use std::fs;
use regex::Regex;
use walkdir::WalkDir;

#[derive(Parser)]
struct Cli {
    #[arg(default_value = ".", short, long)]
    file_path: String,
    query: String,
}

fn main() {
    let cli = Cli::parse();
    let re = Regex::new(&cli.query).unwrap();
    // let paths = fs::read_dir(cli.file_path).unwrap();
    for path in WalkDir::new(cli.file_path) {
        if re.is_match(&path.as_ref().unwrap().path().display().to_string()) {
            println!("{}", path.unwrap().path().display());
        }
    }
}
