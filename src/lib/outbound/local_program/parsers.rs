use regex::{self, Regex};
use crate::domain::game_manager::models::{Position, PlayerInformations};
use thiserror::Error;
use std::num::ParseIntError;
use lazy_static::lazy_static;

#[derive(Debug, Error)]
pub enum ParsePositionError {
    #[error("regular expression failed to compile")]
    InvalidRegex(#[from] regex::Error),
    #[error("player move format is invalid")]
    InvalidFormat,
    #[error("move's coordinates are invalid: `{0}`")]
    InvalidCoordinates(#[from] ParseIntError),
}

pub fn parse_position(
    s: &str
) -> Result<Position, ParsePositionError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+),(\d+)$")
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


pub fn parse_player_informations(
    s: &str
) -> PlayerInformations {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"([\w\-\_]+="([^"]*)""#)
            .expect("About info regular expression failed to initiate itself!");
    }

    PlayerInformations {
        info: RE.captures_iter(s)
            .map(|caps| (caps[1].to_string(), caps[2].to_string()))
            .collect()
    }
}
