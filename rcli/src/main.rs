use crate::cli::base64_opts::Base64SubCommand;
use crate::cli::{Opts, SubCommand};
use crate::process::base64_processor::{base64_decode, base64_encode};
use clap::Parser;
use process::csv_processor::process_csv;
use serde::{Deserialize, Serialize};

mod cli;
mod process;

#[derive(Serialize, Deserialize, Debug)]
struct People {
    #[serde(rename = "id")]
    id: u16,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "age")]
    age: u8,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            process_csv(opts)?;
        }
        SubCommand::Base64(opts) => match opts {
            Base64SubCommand::Encoder(opts) => {
                base64_encode(opts)?;
            }
            Base64SubCommand::Decoder(opts) => {
                base64_decode(opts)?;
            }
        },
    }

    Ok(())
}
