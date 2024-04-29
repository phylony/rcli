use std::{fs::File,io::Read};
use base64::{engine::general_purpose::STANDARD,Engine as _};

pub fn process_encode(input: &str)->anyhow::Result<()>{
    let mut reader :Box<dyn Read>= read_data(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(& mut buf)?;

    let encode=STANDARD.encode(buf);
    println!("{}",encode);
    Ok(())
}

pub fn process_decode(input: &str)->anyhow::Result<()>{
    let mut reader :Box<dyn Read>= read_data(input)?;
    let mut buf = String::new();
    reader.read_to_string(& mut buf)?;
    let buf=buf.trim();
    let decode=STANDARD.decode(buf);
    println!("{:?}",String::from_utf8(decode?));
    Ok(())
}

fn read_data(input: &str)->anyhow::Result<Box<dyn Read>>{
    let reader :Box<dyn Read>= if input =="-"{
        Box::new( std::io::stdin())
    }else{
        Box::new( File::open(input)?)
    };
    Ok(reader)
}
