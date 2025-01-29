use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Cli {
    #[arg(long)]
    #[arg(default_value="INFO")]
    pub log_level: String,

    #[arg(long)]
    pub black_file: PathBuf,

    #[arg(long)]
    pub white_file: PathBuf,
}


