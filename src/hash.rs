use digest::{Digest, DynDigest};
use sha2::Sha256;

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


