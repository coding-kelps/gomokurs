use std::path::PathBuf;
use regex::Regex;
use clap::{Parser, Subcommand, Args};
use thiserror::Error;

#[derive(Parser, Debug, Clone)]
#[clap(disable_help_flag = true)]
struct Cli {
    #[arg(long)]
    #[arg(default_value="INFO")]
    log_level: String,
}

#[derive(Parser, Debug, Clone)]
#[clap(disable_help_flag = true)]
struct PlayerConfigCli {
    #[command(subcommand)]
    player: PlayerConfig,
}

#[derive(Subcommand, Debug, Clone)]
pub enum PlayerConfig {
    Local(LocalConfig),
}

#[derive(Args, Debug, Clone)]
pub struct LocalConfig {
    #[arg(long)]
    pub binary: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Configuration {
    pub log_level: String,
    pub black_player: PlayerConfig,
    pub white_player: PlayerConfig,
}

impl TryFrom<std::env::Args> for Configuration {
    type Error = Error;

    fn try_from(
        args: std::env::Args,
    ) -> Result<Self, <Self as TryFrom<std::env::Args>>::Error> {
        let args_vec = args.collect::<Vec<String>>();
        let args_string = args_vec.join(" ");
        let protocols = vec!["local"].join("|");

        let re = Regex::new(format!("(.*)(({}).*) (({}).*)", protocols, protocols).as_str())
            .expect("regex compilation failed!");

        match re.captures(&args_string) {
            Some(caps) => {
                let manager_args = caps[1].trim().split(" ");
                let black_args = vec![args_vec[0].as_str()].into_iter().chain(caps[2].split(" "));
                let white_args = vec![args_vec[0].as_str()].into_iter().chain(caps[4].split(" "));
        
                let manager_options = Cli::try_parse_from(manager_args)
                    .map_err(|e| Error::FailedArgumentParsing {
                        cli_section: String::from("manager's options"), error: e
                    })?;
                let black_config = PlayerConfigCli::try_parse_from(black_args)
                    .map_err(|e| Error::FailedArgumentParsing {
                        cli_section: String::from("black player configuration"), error: e
                    })?;
                let white_config = PlayerConfigCli::try_parse_from(white_args)
                    .map_err(|e| Error::FailedArgumentParsing {
                        cli_section: String::from("white player configuration"), error: e
                    })?;
        
                Ok(Self {
                    log_level: manager_options.log_level,
                    black_player: black_config.player,
                    white_player: white_config.player,
                })
            }
            None => Err(Error::UnrecognizedPattern),
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("regex compilation error: {0}")]
    FailedRegexCompilation(#[from] regex::Error),
    #[error("unrecognized CLI pattern")]
    UnrecognizedPattern,
    #[error("failed to parse {cli_section} CLI arguments: {error}")]
    FailedArgumentParsing{
        cli_section: String,
        error: clap::error::Error,
    },
}
