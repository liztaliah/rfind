use clap::Parser;
use std::fs;
use regex::Regex;

#[derive(Parser)]
struct Cli {
    #[arg(default_value = ".")]
    file_path: String,
    query: String,
}

fn main() {
    let cli = Cli::parse();
    let re = Regex::new(&cli.query).unwrap();
    let paths = fs::read_dir(cli.file_path).unwrap();
    for path in paths {
        if re.is_match(&path.as_ref().unwrap().path().display().to_string()) {
            println!("{}", path.unwrap().path().display());
        }
    }
}
