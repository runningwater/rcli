mod base64;
mod csv;
mod genpass;
mod http;
mod text;

pub use self::base64::{Base64DecodeOpts, Base64EncodeOpts, Base64Format, Base64SubCommand};
use clap::Parser;
pub use csv::*;
pub use genpass::*;
pub use http::*;
use std::path::{Path, PathBuf};
pub use text::*;

use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about = "Some useful tools", long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Encode or Decode base64 string")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Message Sign, or Verify")]
    Text(TextSubCommand),
    #[command(subcommand, about = "Serve a directory over HTTP")]
    Http(HttpSubCommand),
}

pub fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exists")
    }
}

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exits"), Err("File does not exists"));
    }
}
