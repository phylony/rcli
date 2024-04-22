use std::{fs, path::Path, result};
use  csv::Reader;
use serde::{Serialize,Deserialize};

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

pub fn process_csv(input: &str, output: &str)->anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret =Vec::with_capacity(128);

    for result in reader.deserialize()
    {
        let record: Player = result?;
        ret.push(record);
    }
    let json=serde_json::to_string_pretty(&ret)?;
    fs::write(output,json)?;
    Ok(())
}