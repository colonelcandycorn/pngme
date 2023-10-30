use args::{MyArgs, Commands};
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = MyArgs::parse();

    match cli.command {
        Commands::Encode(val) => commands::encode(val)?,
        Commands::Decode(val) => commands::decode(val)?,
        Commands::Remove(val) => commands::remove(val)?,
        Commands::Print(val) => commands::print(val)?,
    }

    Ok(())
}
