use clap::Parser;
use std::path::Path;

mod csv;
pub use csv::{CsvOpts,OutputFormat};
mod psw;
pub use psw::PswOpts;
mod base64;

pub use base64::{Base64DecodeOpts,Base64SubCommand,Base64EncodeOpts};


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
    #[command(name="psw",about="gen psw")]
    Psw(PswOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),

}

fn verify_filepath(filename : & str) -> Result<String,&'static str>{
    if filename =="-" || Path::new(filename).exists(){
        Ok(filename.into())
    }else {
        Err("File Not Exist")
    }
}

// #[cfg(test)]
// mod tests{

//     use super::*;
//     #[test]
//     fn test_v_inputfile(){
//         assert_eq!(verify_filepath("-"), Ok("-".into()));
//         assert_eq!(verify_filepath("Cargo.toml"), Ok("Cargo.toml".into()));

//     }
// }
