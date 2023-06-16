use clap::ArgMatches;
use cryptea::{cli, config::Config, Hash};

use std::path::PathBuf;

fn main() {
    let matches = cli().get_matches();

    let _ = match matches.subcommand() {
        Some(("crack", sub_matches)) => {
            let file_path = sub_matches
                .get_one::<PathBuf>("PATH")
                .expect("file path")
                .to_owned();
            let hash_value = sub_matches
                .get_one::<String>("HASH")
                .expect("hash value")
                .to_owned();
            let hash_type = get_custom_type(sub_matches);

            let cfg = Config {
                file_path,
                hash_value,
                hash_type,
            };
            Hash::search(cfg);
        }
        Some(("parse", sub_matches)) => {
            let hash_type = get_custom_type(sub_matches);

            if let Some(hash_value) = sub_matches.get_one::<String>("VALUE") {
                let hash_parsed = Hash::parse_hash_value(hash_value, &hash_type);
                println!("{:?}: {}", hash_parsed, hash_value);
            } else {
                eprint!("Error on parsing")
            }
        }
        Some(("example", sub_matches)) => {
            print!("Sarve");
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    };
}

fn get_custom_type(sub_matches: &ArgMatches) -> Hash {
    // if let Some(custom_type) = sub_matches.get_flag::<String>("TYPE") {
    //     Hash::parse_hash(custom_type)
    // } else {
    Hash::SHA256
    // }
}
