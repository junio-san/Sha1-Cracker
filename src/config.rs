use crate::Hash;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Config {
    pub file_path: PathBuf,
    pub hash_value: String,
    pub hash_type: Hash,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            file_path: "./wordlist.txt".into(),
            hash_value: "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824".into(),
            hash_type: Hash::SHA256,
        }
    }
}

impl Config {
    pub fn build_test() -> Config {
        Config::default()
    }
    pub fn set_hash_type(&mut self, new_type: Hash) {
        self.hash_type = new_type;
    }
    pub fn get_hash_type(&self) -> Hash {
        self.hash_type.clone()
    }

    fn new(file_path: PathBuf, hash_value: String, hash_type: Hash) -> Config {
        Self {
            file_path,
            hash_value,
            hash_type,
        }
    }
}
