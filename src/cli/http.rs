use std::path::PathBuf;

use clap::Parser;

use super::verify_path;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "Serve a directory over HTTP")]
    Server(HttpServerOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServerOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub directory: PathBuf,
    #[arg(short, long, default_value = "8080")]
    pub port: u16,
}
