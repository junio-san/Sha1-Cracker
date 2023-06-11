pub mod config;

#[allow(unused_imports)]
use std::io::BufRead;
use sha2::{Sha256, Digest};

// Search through file lines checking if matches hash input
pub fn search(content: String, hash_value: &str) {
    let val = content
        .lines()
        .find(|line| {

            let hash = hex::encode(Sha256::digest(line));
            hash_value == hash
        });

    match val {
        Some(pass) => print!("Password Found: {pass}"),
        None => println!("Password not found"),
    };
}
