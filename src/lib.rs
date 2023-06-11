pub mod config;

use hex::encode;
use sha2::{Sha256, Digest};

// Search through file lines checking if matches hash input
pub fn search(content: String, hash_value: String) {
    let val = content
        .lines()
        .rfind(|line|{
            let hash = Sha256::digest(line);
            hash_value == encode(hash)
        } );

    match val {
        Some(pass) => print!("Password Found: {pass}"),
        _ => println!("No password found"),
    };
}
