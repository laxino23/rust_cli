mod opts;
mod process;

pub use opts::{GenPassOpts, Opts, SubCommand};
pub use process::{csv_convert::process_csv, gen_pass::process_gen_pass};
