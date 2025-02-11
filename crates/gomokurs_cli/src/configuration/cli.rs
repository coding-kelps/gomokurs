use std::path::PathBuf;
use clap::Parser;
use gomokurs_coordinator::domain::coordinator::models::Mode;

fn parse_mode(s: &str) -> Result<Mode, String> {
    match s.to_lowercase().as_str() {
        "single" => Ok(Mode::SingleGame),
        "loop"   => Ok(Mode::Loop),
        _ => Err(format!("Invalid mode: {}", s)),
    }
}

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

    #[arg(long)]
    #[arg(default_value="single")]
    #[arg(value_parser = parse_mode)]
    pub mode: Mode,
}


