use shars::{search, config::Config};
use std::{process, fs};

fn main() {
        let config = Config::build().unwrap_or_else(|err| {
            eprintln!("Error while parsing arguments: {err}");
            process::exit(1);
        });

        let content = fs::read_to_string(config.file_path).unwrap();
        search(content, &config.hash_value)
}
