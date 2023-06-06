use std::env;
use std::{
    error::Error,
};

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 { println!
        ("Use:"); println!
        ("sha1-cracker: <wordlist.txt> <sha1_hash>");       
        return Ok(());
    }

    let hash_to_crack: &str = args[2].trim(); if
    hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is invalid".into());
    }
    Ok(())
}

