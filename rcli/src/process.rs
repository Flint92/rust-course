use crate::opts::CsvOpts;
use csv::Reader;
use serde::de::DeserializeOwned;
use std::fs;

pub fn process_csv<T: serde::Serialize + DeserializeOwned>(
    opts: CsvOpts,
    ret: &mut Vec<T>,
) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(&opts.input)?;
    for result in reader.deserialize() {
        let item: T = result?;
        ret.push(item);
    }

    let json = serde_json::to_string(&ret)?;
    fs::write(&opts.output, json)?;
    Ok(())
}
