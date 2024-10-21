use clap::Parser;
use colored::Colorize;
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
            let string_path = &path.unwrap().path().display().to_string();
            let output: Vec<&str> = string_path.as_str().split(&cli.query).collect();
            println!("{}{}{}", output[0], cli.query.red(), output[1]);
        }
    }
}
