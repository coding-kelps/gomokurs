pub use crate::domain::board_state_manager::models::{PlayerColor, Position};
use crate::domain::game_manager::ports::PlayerNotifier;
use std::collections::HashMap;
use thiserror::Error;
use std::path::PathBuf;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Player<N>
where
    N: PlayerNotifier
{
    pub color: PlayerColor,
    pub ready: bool,
    pub description: Option<PlayerDescription>,
    pub notifier: Arc<N>,
}

#[derive(Debug, Clone)]
pub enum PlayerAction {
    Ok,
    Play(Position),
    Description(PlayerDescription),
    Unknown(String),
    Error(String),
    Message(String),
    Debug(String),
    Suggestion(Position),
}

#[derive(Debug, Clone)]
pub enum RelativeField {
    OwnStone,
    OpponentStone,
}

impl fmt::Display for RelativeField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match &self {
            RelativeField::OwnStone => write!(f, "1"),
            RelativeField::OpponentStone => write!(f, "2"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RelativeTurn {
    pub position: Position,
    pub field: RelativeField,
}

impl fmt::Display for RelativeTurn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "{},{}", self.position,self.field)
    }
}

#[derive(Debug, Clone)]
pub struct PlayerDescription {
    pub info: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum Information {
    TimeoutTurn(u64),
    TimeoutMatch(u64),
    MaxMemory(u64),
    TimeLeft(u64),
    GameType(u8),
    Rule(u8),
    Evaluate{
        x: i32,
        y: i32,
    },
    Folder(PathBuf),
}

impl fmt::Display for Information {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Information::TimeoutTurn(t) => write!(f, "timeout_turn {}", t),
            Information::TimeoutMatch(t) => write!(f, "timeout_match {}", t),
            Information::MaxMemory(m) => write!(f, "max_memory {}", m),
            Information::TimeLeft(t) => write!(f, "time_left {}", t),
            Information::GameType(t) => write!(f, "game_type {}", t),
            Information::Rule(r) => write!(f, "rule {}", r),
            Information::Evaluate{x, y} => write!(f, "evaluate {},{}", x, y),
            Information::Folder(p) => {
                let path = p.clone()
                    .into_os_string()
                    .into_string()
                    .expect("failed to convert persistent folder path into str");

                write!(f, "folder {}", path)
            },
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to notify `{color}`: `{error}`")]
    NotifyError{
        error: NotifyError,
        color: PlayerColor,
    },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum NotifyError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
