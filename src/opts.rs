use anyhow;
use clap::{builder::Str, Parser, Subcommand};
use core::fmt;
use std::{path::Path, result, str::FromStr};

//rcli csv -i input.csv -o output.json --header -d ','
#[derive(Debug,Parser)]
#[command(name = "rcli" , version, author, about, long_about = None)]

pub struct Opts{
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug,Parser)]
pub enum SubCommand {
    #[command(name="csv",about ="show csv")]
    Csv(CsvOpts),
}
#[derive(Debug,Clone,Copy)]
pub enum  OutputFormat {
    Json,
    Yaml,
    Toml,    
}

#[derive(Debug,Parser)]
pub struct CsvOpts{
    #[arg(short,long ,value_parser = verify_filepath)]
    pub input: String,

    #[arg(short,long)]
    pub output: Option<String>,

    #[arg(short,long,value_parser =parse_format, default_value = "json")]
    pub format: OutputFormat,

    #[arg(short,long, default_value_t=',')]
    pub delimiter: char,

    #[arg( long, default_value_t = true)]
    pub header: bool,
}

fn verify_filepath(filename : & str) -> Result<String,&'static str>{
    if Path::new(filename).exists(){
        Ok(filename.into())
    }else {
        Err("File Not Exist".into())
    }
}

fn parse_format(format : &str) ->Result<OutputFormat,anyhow::Error>{
    
    format.parse::<OutputFormat>()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat)->Self{
        match format{
            OutputFormat::Json =>"json",
            OutputFormat::Yaml =>"yaml",
            OutputFormat::Toml =>"tome",
        }
    }
}

impl  FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str)-> Result<Self,Self::Err>{
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" =>Ok(OutputFormat::Yaml),
            "toml" =>Ok(OutputFormat::Toml),
            _=> Err(anyhow::anyhow!("Invalid Format!")),
    
        }
    }
}

impl  fmt::Display for OutputFormat {
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        write!(f,"{}",Into::<&str>::into(*self))
    }
}