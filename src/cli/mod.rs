use std::path::{Path, PathBuf};

use clap::Parser;

use crate::CmdExecutor;

use self::csv::CsvOpts;
use self::genpass::GenPassOpts;
pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::OutputFormat,
    http::HttpSubCommand,
    text::{TextSignFormat, TextSubCommand},
};

mod base64;
mod csv;
mod genpass;
mod http;
mod text;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about = "Some useful tools", long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
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

impl CmdExecutor for SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            SubCommand::Csv(opts) => opts.execute().await,
            SubCommand::GenPass(opts) => opts.execute().await,
            SubCommand::Base64(cmd) => cmd.execute().await,
            SubCommand::Text(cmd) => cmd.execute().await,
            SubCommand::Http(cmd) => cmd.execute().await,
        }
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
