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
    file_path: Option<String>,

    /// Valid Sha256 Hash for parsing
    #[arg(short='s', long="sha256")]
    hash_value: Option<String>,

    /// Uses example files to test file
    #[arg(short='t',long, default_value_t=true )]
    test: bool,
}

#[derive(Clone)]
pub struct Config {
    pub file_path: String,
    pub hash_value: String,
}

static SHA1_LENGTH: u8 = 64;

impl Config {
    fn new(file_path: String,hash_value:  String) -> Config {
       Self{file_path, hash_value}
    }
    pub fn build() -> Result<Config, &'static str> {
        let default_path: String = "./wordlist.txt".into();
        let default_hash: String = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824".into();

        let Args {file_path, hash_value, test} = Args::parse();
        let config = match test {
            true => Config::new(default_path, default_hash),
            _ => Config {file_path: file_path.unwrap(), hash_value: hash_value.unwrap() }
        };

        if config.hash_value.len() as u8 != SHA1_LENGTH {
            return Err("Hash inserted is too short")
        }

        Ok(config)
    }
}
