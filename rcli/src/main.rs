use clap::Parser;
use rcli::{Opts, SubCommand, process_csv};
use serde::{Deserialize, Serialize};

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
            let mut ret: Vec<People> = Vec::with_capacity(128);
            process_csv(opts, &mut ret)?;
        }
    }

    Ok(())
}
