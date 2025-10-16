use clap::Parser;
use std::{path::Path, str::FromStr};

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
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}
impl From<OutputFormat> for &str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
        }
    }
}

impl TryFrom<&str> for OutputFormat {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            _ => Err(anyhow::format_err!(
                "Unsupported output format: {}. Supported formats are: json, yaml, toml",
                value
            )),
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        OutputFormat::try_from(s)
    }
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(
        short,
        long,
        help = "Input file path, if not specified, read from stdin",
        value_parser = verify_input_file,
    )]
    pub input: String,

    #[arg(
        short,
        long,
        help = "Output file path, if not specified, write to stdout"
    )]
    pub output: Option<String>,

    #[arg(
        long,
        help = "Output format, default is json, options: json, yaml, toml",
        value_parser = parse_format,
        default_value = "Json"
    )]
    pub format: OutputFormat,

    #[arg(
        long, // do not have short version  because it conflicts with help
        help = "Whether the CSV has header, default is false",
        default_value_t = true
    )]
    pub header: bool,

    #[arg(
        short,
        long,
        help = "Delimiter, default is comma",
        default_value_t = ',' // or default_value = ",".into()
    )]
    pub delimiter: char,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err(format!("Input file {} does not exist", filename))
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, String> {
    // OutputFormat::try_from(format).map_err(|e| e.to_string()) // try_from is from TryFrom impl
    format.parse().map_err(|e: anyhow::Error| e.to_string()) // parse is from FromStr
}
