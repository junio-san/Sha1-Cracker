use clap::ArgMatches;
use cryptea::{cli, config::Config, Hash};

use std::path::PathBuf;

fn main() {
    let matches = cli().get_matches();

    let _ = match matches.subcommand() {
        Some(("find", sub_matches)) => {
            let mut cfg = Config::default();

            cfg.file_path = sub_matches
                .get_one::<PathBuf>("file")
                .expect("file path")
                .to_owned();

            cfg.hash_value = sub_matches
                .get_one::<String>("hash")
                .expect("hash value")
                .to_owned();

            cfg.hash_type = get_custom_type(sub_matches);

            Hash::search(cfg);
        }
        Some(("parse", sub_matches)) => {
            let hash_type = get_custom_type(sub_matches);

            if let Some(hash_value) = sub_matches.get_one::<String>("value") {
                let hash_parsed = Hash::parse_hash_value(hash_value, &hash_type);
                println!("{}", hash_parsed);
            } else {
                eprint!("Error on parsing")
            }
        }
        Some(("example", _sub_matches)) => {
            Hash::search(Config::default());
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    };
}

fn get_custom_type(sub_matches: &ArgMatches) -> Hash {
    if let Some(custom_type) = sub_matches.get_one::<String>("type") {
        Hash::parse_hash(custom_type)
    } else {
        Hash::SHA256
    }
}
