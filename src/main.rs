use clap::Parser;
use std::fs;

#[derive(Parser)]
struct CLI {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = CLI::parse();

    let file_content = fs::read_to_string(&args.path).expect("couldn't read the file");

    for line in file_content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line)
        }
    }
}
