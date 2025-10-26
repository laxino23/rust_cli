use std::fs;

use clap::Parser;

use rcli::{
    Base64SubCommand, HttpSubCommand, Opts, SubCommand,
    TextSignFormat::{Blake3, Ed25519},
    TextSubCommand, process_csv, process_decode, process_encode,
    process_gen_pass, process_http_server, process_key_generate, process_sign,
    process_verify,
};

// cl takes arguments from command line
// csv -> (deserialize) -> struct -> (serialize) -> json

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化 tracing_subscriber 日志格式化器，用于日志输出
    tracing_subscriber::fmt::init();
    let opts: Opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output: String = opts.output.clone().unwrap_or_else(|| {
                format!("output.{}", <&str>::from(opts.format)) // from impl
            });
            process_csv(&opts.input, &output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            let result = process_gen_pass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.numbers,
                opts.symbols,
            )?;
            println!("{}\n{}\n{}", result.0, result.1, result.2);
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let encoded = process_encode(&opts.input, opts.format)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format)?;
                //NOTE the result may not be valid UTF-8 string, we assume it is for display
                println!("{}", String::from_utf8_lossy(&decoded));
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let signed = process_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", signed);
            }
            TextSubCommand::Verify(opts) => {
                let verified = process_verify(
                    &opts.input,
                    &opts.key,
                    &opts.signature,
                    opts.format,
                )?;
                println!("{}", verified);
            }
            TextSubCommand::KeyGenerate(opts) => {
                let key = process_key_generate(opts.format)?;
                match opts.format {
                    Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        fs::write(name, &key[0])?;
                    }
                    Ed25519 => {
                        let name = opts.output.join("ed25519_private.key");
                        fs::write(&name, &key[0])?;
                        let name = opts.output.join("ed25519_public.key");
                        fs::write(&name, &key[1])?;
                    }
                }
            }
        },
        SubCommand::Http(subcmd) => match subcmd {
            HttpSubCommand::Server(opts) => {
                process_http_server(opts.directory, opts.port).await?;
            }
        },
    }
    Ok(())
}
