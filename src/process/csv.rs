use std::{collections::HashMap, fs};
use  csv::Reader;
use serde::{Serialize,Deserialize};
use serde_json::Value;

use crate::cli::OutputFormat;

#[derive(Debug,Deserialize,Serialize)]
pub struct Player{
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename ="Position")]
    position: String,
    #[serde(rename="DOB")]
    dob: String,
    #[serde(rename="Nationality")]
    nationality:String,
    #[serde(rename="Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: String, format:OutputFormat)->anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret =Vec::with_capacity(128);
    let headers =reader.headers().cloned()?;
    let mut hash=HashMap::with_capacity(1);

    for result in reader.records()
    {
        let record = result?;
        let json_val =headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_val);
    }

    hash.insert("parent".to_string(),ret);

    let content=match format{
        OutputFormat::Json =>serde_json::to_string_pretty(&hash)?,
        OutputFormat::Yaml =>serde_yaml::to_string(&hash)?,
        OutputFormat::Toml => toml::to_string(&hash)?,
    };
    fs::write(output,content)?;
    Ok(())
}
