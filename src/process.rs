use std::{fs, path::Path, result};
use  csv::Reader;
use serde::{Serialize,Deserialize};
use serde_json::Value;

use crate::opts::OutputFormat;

#[derive(Debug,Deserialize,Serialize)]
pub struct Player{
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename ="Position")]
    postion: String,
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
    for result in reader.records()
    {
        let record = result?;
        let json_val =headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_val);
    }
    let json=serde_json::to_string_pretty(&ret)?;
    fs::write(output,json)?;
    Ok(())
}