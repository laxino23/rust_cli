use core::fmt;
use std::{path::PathBuf, str::FromStr};

use super::{verify_file, verify_path};
use clap::Parser;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign a text")]
    Sign(TextSignOpts),
    #[command(about = "Verify a text")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a key pair")]
    KeyGenerate(KeyGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_file)]
    pub key: String,

    #[arg(long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_file)]
    pub key: String,

    #[arg(short, long)]
    pub signature: String,

    #[arg(long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct KeyGenerateOpts {
    #[arg(long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

impl FromStr for TextSignFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(format!("Invalid format: {}", s)),
        }
    }
}
impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}
impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: &str = (*self).into();
        write!(f, "{}", s)
    }
}

fn parse_format(format: &str) -> Result<TextSignFormat, String> {
    match format.to_lowercase().as_str() {
        "blake3" => Ok(TextSignFormat::Blake3),
        "ed25519" => Ok(TextSignFormat::Ed25519),
        _ => Err(format!("Invalid text sign format: {}", format)),
    }
}
