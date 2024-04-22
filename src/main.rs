
use anyhow;
use rcli::{Opts,process_csv,SubCommand};
use clap::Parser;


fn main()->anyhow::Result<()> {
    println!("Hello, world!");
    let opts:Opts =Opts::parse();
    println!("Outpt:{:?}",opts);

    match opts.cmd {
        SubCommand::Csv(opts) => {
            process_csv(&opts.input, &opts.output)?;
        }

    };
    Ok(())
}
