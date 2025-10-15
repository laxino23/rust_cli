// rcli csv -i input.csv -o output.json -- header -d ','
use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

// cl takes arguments from command line
// csv -> (deserialize) -> struct -> (serialize) -> json
fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            process_csv(&opts.input, &opts.output)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
