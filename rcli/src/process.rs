use crate::opts::{CsvOpts, OutputFormat};
use csv::Reader;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

pub fn process_csv(opts: CsvOpts) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(&opts.input)?;
    let mut ret = Vec::with_capacity(128);

    let headers = reader.headers()?.clone();

    for result in reader.records() {
        let record = result?;
        let iter = headers.iter().zip(record.iter());

        let val = iter.collect::<Value>();

        ret.push(val);
    }

    let output = if let Some(output) = opts.output {
        output
    } else {
        format!("output.{}", opts.format)
    };

    let content = match opts.format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        OutputFormat::Toml => {
            let mut data = HashMap::new();
            data.insert("records", ret);
            toml::to_string(&data)?
        }
    };

    fs::write(&output, content)?;
    Ok(())
}
