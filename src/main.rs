use rcli::{Opts,process_csv,SubCommand,process_genpass};
use clap::Parser;
use rcli::{process_decode,process_encode};

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
        },
        SubCommand::Psw(opts) =>{
            let result= process_genpass(opts.len,opts.upper,opts.lower,opts.number,opts.symbol);

            format!("{:?}",result?);
        },
        SubCommand::Base64(subcmd)=>match  subcmd {
            rcli::Base64SubCommand::Encode(opts) =>{
                print!("{:?}",opts);
                process_encode(&opts.input)?;
            },
            rcli::Base64SubCommand::Decode(opts)=>{
                print!("{:?}",opts);
                process_decode(&opts.input)?;
            }
        },


    };
    Ok(())
}
