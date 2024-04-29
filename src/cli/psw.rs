use clap::Parser;

#[derive(Debug,Parser)]
pub struct PswOpts{
    #[arg(long ,default_value_t = 16)]
    pub len: u8,

    #[arg(long, default_value_t = true)]
    pub upper: bool,

    #[arg(long, default_value_t = true)]
    pub lower: bool,

    #[arg(long, default_value_t= true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}
