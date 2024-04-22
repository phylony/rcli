use clap::{builder::Str, Parser, Subcommand};
use std::{path::Path, result};

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

#[derive(Debug,Parser)]
pub struct CsvOpts{
    #[arg(short,long ,value_parser = verify_filepath)]
    pub input: String,

    #[arg(short,long ,default_value="output.json")]
    pub output: String,

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
