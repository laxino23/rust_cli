mod cli;
mod process;

// cli
pub use cli::{Opts, SubCommand};
// cli sub modules
pub use cli::{base64::*, csv::*, genpass::*};

// process
pub use process::{b64::*, csv_convert::process_csv, gen_pass::process_gen_pass};
