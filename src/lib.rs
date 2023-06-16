use std::fs;
use std::path::PathBuf;

use clap::Command;
use clap::{arg, value_parser};
pub mod config;
use config::Config;
use digest::{Digest, DynDigest};
use sha2::Sha256;

pub enum CLIOptions {
    Find(Config),
    Parse(String),
    Example,
}

#[derive(Clone)]
pub enum Hash {
    SHA256,
    SHA224,
    SHA1,
    SHA384,
    SHA512,
}

impl Hash {
    pub fn search(config: Config) {
        // Search through file lines until matches hash input
        if !config.file_path.is_file() {
            print!("invalid file");
            return;
        }
        let path = fs::read_to_string(config.file_path).expect("Error loading file");

        match path
            .lines()
            .find(|line| config.hash_value == Self::parse_hash_value(line, &config.hash_type))
        {
            Some(pass) => {
                println!("Password found: {pass}");
            }
            None => {
                println!("Password not found");
            }
        }
    }

    pub fn parse_hash(item: &str) -> Self {
        let value = match item {
            "sha1" => Hash::SHA1,
            "sha224" => Hash::SHA224,
            "sha256" => Hash::SHA256,
            "sha384" => Hash::SHA384,
            "sha512" => Hash::SHA512,
            _ => unimplemented!("unsupported digest: {}", item),
        };

        value
    }

    pub fn parse_hash_value(val: &str, hash_type: &Hash) -> String {
        let val = Self::use_hasher(val.as_bytes(), hash_type);
        hex::encode(Sha256::digest(val))
    }

    fn select_hasher(data: &Hash) -> Box<dyn DynDigest> {
        match data {
            Hash::SHA1 => Box::new(sha1::Sha1::default()),
            Hash::SHA224 => Box::new(sha2::Sha224::default()),
            Hash::SHA256 => Box::new(sha2::Sha256::default()),
            Hash::SHA384 => Box::new(sha2::Sha384::default()),
            Hash::SHA512 => Box::new(sha2::Sha512::default()),
        }
    }

    fn use_hasher(data: &[u8], hash: &Hash) -> Box<[u8]> {
        let mut hasher: Box<dyn DynDigest> = Self::select_hasher(hash);
        hasher.update(data);
        hasher.finalize_reset()
    }
}

/// CLI parses arguments into it
pub fn cli() -> clap::Command {
    Command::new("srs")
        .about("SHA hash family parser and cracker")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("crack")
                .about("Crack hash by matching hash function in a file")
                .arg(
                    arg!(path: [PATH])
                        .num_args(1)
                        .value_parser(value_parser!(PathBuf))
                        .group("FIND"),
                )
                .arg_required_else_help(true)
                .arg(
                    arg!(hash: [HASH])
                        .require_equals(false)
                        .num_args(1)
                        .value_parser(value_parser!(String))
                        .group("FIND"),
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("parse")
                .about("Parse word using SHA256 hash function")
                .arg(arg!(value: <STRING> "String value to convert")),
        )
        .subcommand(Command::new("example").about("Run example with "))
}

// fn custom_type() -> clap::Arg {
//     arg!(type: [TYPE])
//         .value_parser(["md5", "sha1", "sha224", "sha256", "sha384", "sha512"])
//         .num_args(1)
//         .require_equals(false)
//         .required(false)
//         .default_value("Sha256")
//         .value_parser(value_parser!(String))
// }
