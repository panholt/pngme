mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use clap::Parser;
use crate::args::{Cli, Commands};
use crate::commands::{encode, decode, remove, print};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Encode { path, chunk_type, message } => encode(path, chunk_type, message),
        Commands::Decode { path, chunk_type } => decode(path, chunk_type),
        Commands::Remove { path, chunk_type } => remove(path, chunk_type),
        Commands::Print { path } => print(path),
    }

}
