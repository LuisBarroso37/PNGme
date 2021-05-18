#![allow(unused_imports)]
#![allow(dead_code)]

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use structopt::StructOpt;

/// Holds any kind of error
pub type Error = Box<dyn std::error::Error>;

/// Holds a `Result` of any kind of error
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let opt = args::Opt::from_args();
    commands::run(opt.subcommand)
}
