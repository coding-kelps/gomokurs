use regex::{self, Regex};
use crate::domain::coordinator::models::{Position, PlayerAction, PlayerMetadata};
use thiserror::Error;
use std::num::ParseIntError;
use lazy_static::lazy_static;
use anyhow::anyhow;

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("unknown command")]
    UnknownCommand,
    #[error("parsing error: `{0}`")]
    ParsingError(#[from] anyhow::Error),
}

pub fn parse_input(input: String) -> Result<PlayerAction, ParseInputError> {
    lazy_static! {
        static ref RE_OK: Regex = Regex::new(r"^OK$")
            .expect("failed to initiate ok command regex!");
        static ref RE_PLAY: Regex = Regex::new(r"^\d+,\d+$")
            .expect("failed to initiate play command regex!");
        static ref RE_DESC: Regex = Regex::new(r#"[\w\-\_]+="[^"]*""#)
            .expect("failed to initiate player description command regex!");
        static ref RE_UNK: Regex = Regex::new(r"UNKNOWN .*")
            .expect("failed to initiate unknown command regex!");
        static ref RE_ERR: Regex = Regex::new(r"ERROR .*")
            .expect("failed to initiate error command regex!");
        static ref RE_MSG: Regex = Regex::new(r"MESSAGE .*")
            .expect("failed to initiate message command regex!");
        static ref RE_DBG: Regex = Regex::new(r"DEBUG .*")
            .expect("failed to initiate debug command regex!");
        static ref RE_SGT: Regex = Regex::new(r"SUGGEST .*")
            .expect("failed to initiate suggestion command regex!");
    }

    if RE_OK.is_match(&input) {
        Ok(PlayerAction::Ready)
    } else if RE_PLAY.is_match(&input) {
        let position = parse_position(&input)
            .map_err(|e| anyhow!(e))?;

        Ok(PlayerAction::Play(position))
    } else if RE_DESC.is_match(&input) {
        let metadata = parse_player_metadata(&input);

        Ok(PlayerAction::Metadata(metadata))
    } else if RE_UNK.is_match(&input) {
        let content = parse_content(&input)
            .map_err(|e| anyhow!(e))?;

        Ok(PlayerAction::Unknown(content))
    } else if RE_ERR.is_match(&input) {
        let content = parse_content(&input)
            .map_err(|e| anyhow!(e))?;

        Ok(PlayerAction::Error(content))
    } else if RE_MSG.is_match(&input) {
        let content = parse_content(&input)
            .map_err(|e| anyhow!(e))?;

        Ok(PlayerAction::Message(content))
    } else if RE_DBG.is_match(&input) {
        let content = parse_content(&input)
            .map_err(|e| anyhow!(e))?;

        Ok(PlayerAction::Debug(content))
    } else {
        Err(ParseInputError::UnknownCommand)
    }
}

#[derive(Debug, Error)]
enum ParsePositionError {
    #[error("regular expression failed to compile")]
    InvalidRegex(#[from] regex::Error),
    #[error("player move format is invalid")]
    InvalidFormat,
    #[error("move's coordinates are invalid: `{0}`")]
    InvalidCoordinates(#[from] ParseIntError),
}

fn parse_position(
    s: &str
) -> Result<Position, ParsePositionError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?:SUGGEST\s*)?(\d+),(\d+)$")
            .expect("Position regular expression failed to initiate itself!");
    }

    match RE.captures(s) {
        Some(caps) => {
            let x = caps[1].parse::<u8>()?;
            let y = caps[2].parse::<u8>()?;

            Ok(Position::new(x, y))
        }
        None => Err(ParsePositionError::InvalidFormat),
    }
}

fn parse_player_metadata(
    s: &str
) -> PlayerMetadata {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"([\w\-\_]+="([^"]*)""#)
            .expect("About info regular expression failed to initiate itself!");
    }

    PlayerMetadata {
        info: RE.captures_iter(s)
            .map(|caps| (caps[1].to_string(), caps[2].to_string()))
            .collect()
    }
}

#[derive(Debug, Error)]
enum ParseContentError {
    #[error("regular expression failed to compile")]
    InvalidRegex(#[from] regex::Error),
    #[error("player move format is invalid")]
    InvalidFormat,
}

fn parse_content(
    s: &str
) -> Result<String, ParseContentError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?:ERROR|UNKNOWN|DEBUG|MESSAGE)\s?(.*)$")
            .expect("Content regular expression failed to initiate itself!");
    }

    match RE.captures(s) {
        Some(caps) => {
            Ok(caps[1].to_string())
        }
        None => Err(ParseContentError::InvalidFormat),
    }
}
