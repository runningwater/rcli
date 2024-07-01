mod base64;
mod csv;
mod genpass;

use std::path::Path;

use self::csv::CsvOpts;
use self::genpass::GenPassOpts;
pub use self::{base64::Base64Format, base64::Base64SubCommand, csv::OutputFormat};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author , about= "Some useful tools", long_about = None)]
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
}

pub fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exists")
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("not-exits"), Err("File does not exists"));
    }
}
