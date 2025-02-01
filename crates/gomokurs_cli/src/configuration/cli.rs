use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Cli {
    #[arg(long)]
    pub black_file: PathBuf,

    #[arg(long)]
    pub white_file: PathBuf,

    #[arg(short, long)]
    #[arg(default_value="30")]
    pub turn_duration: u64,

    #[arg(short, long)]
    #[arg(default_value="180")]
    pub match_duration: u64,

    #[arg(long)]
    #[arg(default_value="INFO")]
    pub log_level: String,
}


