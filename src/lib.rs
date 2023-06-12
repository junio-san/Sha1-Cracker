use {
    std::path::PathBuf,
    clap::{arg, Command, value_parser},
};
mod config;
use config::Config;

static STRING_LENGTH: usize = 64;

pub enum CLIOptions {
    Find(Config),
    Parse(String),
    Type(String),
    Example
}

/// parse arguments into it
pub fn cli() -> Command {
    Command::new("srs")
        .about("A fictional versioning CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("find")
                .about("Find word that matches hash function in a file")
                .arg(arg!(path: [PATH])
                    .num_args(1)
                    .value_parser(value_parser!(PathBuf))
                    .group("FIND")
                )
                .arg_required_else_help(true)
                .arg(arg!(hash: [HASH])
                    .require_equals(false)
                    .num_args(1)
                    .value_parser(value_parser!(String))
                    .group("FIND")
                )
                .arg_required_else_help(true)
                .arg(custom_type())
        )
        .subcommand(
            Command::new("parse")
                .about("Parse word using SHA256 hash function")
                .arg(arg!(value: <VALUE> "Hash value"))
                .arg(custom_type())
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("example")
                .about("Run example with ")
                .arg(arg!(value: <VALUE> "Hash value")
                    .exclusive(true)
                )
        )
}

fn custom_type() -> clap::Arg {
    arg!(type: [TYPE])
        .value_parser(["md5", "sha1", "sha256", "sha512"])
        .num_args(1)
        .require_equals(false)
        .required(false)
        .default_value("Sha256")
        .value_parser(value_parser!(String))
}
