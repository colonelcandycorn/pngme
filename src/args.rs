use clap::{Args, Parser, Subcommand};
use std::path;
#[derive(Debug, Parser)]
#[command(author = "Sarah Dylan")]
#[command(name = "pngme")]
#[command(about = "Encode and Decode Png files", long_about = None)]
pub struct MyArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    // Encode a message 
    #[command(arg_required_else_help = true)]
    #[command(short_flag = 'e')]
    Encode(EncodeCommand),
    #[command(arg_required_else_help = true)]
    #[command(short_flag = 'd')]
    Decode(DecodeCommand),
    #[command(arg_required_else_help = true)]
    #[command(short_flag = 'r')]
    Remove(RemoveCommand),
    #[command(arg_required_else_help = true)]
    #[command(short_flag = 'p')]
    Print(PrintCommand),
}

#[derive(Debug, Args)]
pub struct EncodeCommand {
    pub file_path: path::PathBuf,
    pub chunk: String,
    pub message: String,
    pub output: Option<String>,
}

#[derive(Debug, Args)]
pub struct DecodeCommand {
    pub file_path: String,
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct RemoveCommand {
    pub file_path: String,
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct PrintCommand {
    pub file_path: String,
}
