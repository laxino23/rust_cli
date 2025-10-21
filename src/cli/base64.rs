use std::str::FromStr;

use super::verify_file;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(
    format: &str,
) -> anyhow::Result<Base64Format, anyhow::Error> {
    Base64Format::from_str(format)
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(format: &str) -> Result<Self, Self::Err> {
        match format.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" | "url_safe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!(
                "Invalid base64 format: {}. Use 'standard' or 'urlsafe'",
                format
            )),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl TryFrom<&str> for Base64Format {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Base64Format::from_str(value)
    }
}

impl std::fmt::Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Base64Format::Standard => write!(f, "standard"),
            Base64Format::UrlSafe => write!(f, "urlsafe"),
        }
    }
}
