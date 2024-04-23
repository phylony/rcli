
use std::process::Output;

use anyhow;
use rcli::{Opts,process_csv,SubCommand};
use clap::Parser;


fn main()->anyhow::Result<()> {
    println!("Hello, world!");
    let opts:Opts =Opts::parse();
    println!("Outpt:{:?}",opts);

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output{
                output.clone()
            }else{
                format!("output.{}",opts.format)
            };
            process_csv(&opts.input, output ,opts.format)?;
        }

    };
    Ok(())
}
