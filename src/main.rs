// rcli csv -i input.csv -o output.json -- header -d ','
use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

// cl takes arguments from command line
// csv -> (deserialize) -> struct -> (serialize) -> json
fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output: String = opts.output.clone().unwrap_or_else(|| {
                format!("output.{}", <&str>::from(opts.format)) // from impl
            });
            process_csv(&opts.input, &output, opts.format)?;
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
