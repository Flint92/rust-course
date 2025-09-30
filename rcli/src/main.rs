use clap::Parser;
use cli::csv_opts::{Opts, SubCommand};
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
    }

    Ok(())
}
