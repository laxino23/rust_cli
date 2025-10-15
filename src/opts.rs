use clap::Parser;
use std::path::Path;

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
        help = "Output file path, if not specified, write to stdout",
        default_value = "output.json" // "output.json".into()
    )]
    pub output: String,

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
