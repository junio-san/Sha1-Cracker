use std::string::ToString;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(
    about = "Srs: a basic Sha256 finder. Type `-h` for help or `-test` for a test drive!",
    long_about= None,
    arg_required_else_help(true),
)]
struct Args {
    /// Path of the file to find word
    #[arg(short='p', long="path")]
    file_path: String,

    /// Valid Sha256 Hash for parsing
    #[arg(short='s', long="sha256")]
    hash_value: String,

    /// Uses example files to test file
    #[arg(short='t',long, default_value_t=true )]
    test: bool,
}

#[derive(Clone)]
pub struct Config {
    pub file_path: String,
    pub hash_value: String,
}

static SHA1_LENGTH: u8 = 65;

impl Config {
    fn new(file_path: String,hash_value:  String) -> Config {
       Self{file_path, hash_value}
    }
    pub fn build() -> Result<Config, &'static str> {
        let default_path: String = "./wordlist.txt".into();
        let default_hash: String = "620af73b5a4cb4bc3172c94d595ae3c79ec2c256fd1be4b3b170e3aae0422d98".into();

        let Args {file_path, hash_value, test} = Args::parse();
        let config = match test {
            true => Config::new(default_path, default_hash),
            _ => Config {file_path, hash_value }
        };

        if config.hash_value.len() as u8 != SHA1_LENGTH {
            return Err("Hash inserted is too short")
        }

        Ok(config)
    }
}
