mod cli;
mod process;
mod utils;

// cli
pub use cli::{Opts, SubCommand};
// cli sub modules
pub use cli::{base64::*, csv::*, genpass::*, text::*};

// process
pub use process::{
    b64::*,
    csv_convert::process_csv,
    gen_pass::process_gen_pass,
    text::{process_key_generate, process_sign, process_verify},
};

// utils
pub use utils::*;
