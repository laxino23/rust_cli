use clap::builder::ArgAction::SetFalse;
use clap::Parser;

// MARK - GENPASS OPTIONS
#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(long, help = "Length of the password", default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true, help = "Include uppercase letters", action = SetTrue)]
    #[arg(long = "no-uppercase", action = SetFalse, help = "Exclude uppercase letters")]
    pub uppercase: bool,

    #[arg(long, default_value_t = true, help = "Include lowercase letters", action = SetTrue)]
    #[arg(long = "no-lowercase", action = SetFalse, help = "Exclude lowercase letters")]
    pub lowercase: bool,

    #[arg(long, default_value_t = true, help = "Include numbers", action = SetTrue)]
    #[arg(long = "no-numbers", action = SetFalse, help = "Exclude numbers")]
    pub numbers: bool,

    #[arg(long, default_value_t = true, help = "Include symbols", action = SetTrue)]
    #[arg(long = "no-symbols", action = SetFalse, help = "Exclude symbols")]
    pub symbols: bool,
}
