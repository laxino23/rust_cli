pub mod base64;
pub mod csv;
pub mod genpass;
pub mod text;

use clap::Parser;
use std::path::{Path, PathBuf};

use crate::cli::base64::Base64SubCommand;
use crate::cli::csv::CsvOpts;
use crate::cli::genpass::GenPassOpts;
use crate::cli::text::TextSubCommand;

#[derive(Debug, Parser)] // from macro get traits
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(
        name = "csv",
        about = "Show csv, or convert CSV to other formats"
    )]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
}

pub fn verify_file(filename: &str) -> Result<String, String> {
    // if input is "-", read from stdin
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err(format!("Input file {} does not exist", filename))
    }
}

pub fn verify_path(path: &str) -> Result<PathBuf, String> {
    let path = Path::new(path);
    if path.exists() && path.is_dir() {
        Ok(path.to_path_buf())
    } else {
        Err(format!(
            "Input file {:?} does not exist or is not a directory",
            path
        ))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("Input file * does not exist".into()));
        assert_eq!(verify_file("src/cli/mod.rs"), Ok("src/cli/mod.rs".into()));
        assert!(verify_file("non_existent_file.txt").is_err());
    }
}
