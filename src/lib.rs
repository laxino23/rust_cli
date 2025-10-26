mod cli;
mod process;
mod utils;

// cli
pub use cli::{Opts, SubCommand};
// cli sub modules
pub use cli::{base64::*, csv::*, genpass::*, http::*, text::*};

// process
pub use process::{
    b64::*,
    csv_convert::process_csv,
    gen_pass::process_gen_pass,
    http_serve::process_http_server,
    text::{process_key_generate, process_sign, process_verify},
};

// utils
pub use utils::*;
