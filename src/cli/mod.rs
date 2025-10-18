pub mod base64;
pub mod csv;
pub mod genpass;

use std::path::Path;

use crate::cli::base64::Base64SubCommand;
use crate::cli::csv::CsvOpts;
use crate::cli::genpass::GenPassOpts;
use clap::Parser;

#[derive(Debug, Parser)] // from macro get traits
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show csv, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

pub fn verify_input_file(filename: &str) -> Result<String, String> {
    // if input is "-", read from stdin
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err(format!("Input file {} does not exist", filename))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(
            verify_input_file("*"),
            Err("Input file * does not exist".into())
        );
        assert_eq!(
            verify_input_file("src/cli/mod.rs"),
            Ok("src/cli/mod.rs".into())
        );
        assert!(verify_input_file("non_existent_file.txt").is_err());
    }
}
