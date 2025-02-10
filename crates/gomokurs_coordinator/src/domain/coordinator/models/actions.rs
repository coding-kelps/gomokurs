pub use gomokurs_game_engine::domain::game_engine::models::{PlayerColor, Position, GameEnd, Error as GameEngineError};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fmt;

/// Represents an action initiated by a player.
///
/// Actions may originate from the player program and can include invalid inputs.
/// For more information, refer to the
/// [`Gomocup Protocol`](https://plastovicka.github.io/protocl2en.htm).
#[derive(Debug, Clone)]
pub enum PlayerAction {
    /// The player declares readiness to play.
    Ready,
    /// The player makes a move at the specified board position.
    Play(Position),
    /// Provides metadata about the player.
    Metadata(PlayerMetadata),
    /// Indicates that the player did not recognize the last action.
    Unknown(String),
    /// Indicates an error encountered by the player.
    Error(String),
    /// Sends a message to the other player.
    Message(String),
    /// Debugging information sent by the player.
    Debug(String),
    /// Suggests a move to the manager.
    Suggestion(Position),
}

/// Metadata about a player as a collection of key-value pairs.
#[derive(Debug, Clone)]
pub struct PlayerMetadata {
    pub info: HashMap<String, String>,
}

/// Represents the state of a cell from the player's perspective.
///
/// Used when the manager sends the `BOARD` command to a player.
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

/// Represents a game turn from the player's perspective.
///
/// Used when the manager sends the `BOARD` command to a player.
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

/// Represents information that can be sent by the manager to a player.
#[derive(Debug, Clone)]
pub enum Information {
    /// Time limit for each turn in milliseconds.
    TimeoutTurn(u64),
    /// Total time limit for the match in milliseconds.
    TimeoutMatch(u64),
    /// Memory limit for the player program in bytes (optional for local
    /// programs).
    MaxMemory(u64),
    /// Remaining time for the match in milliseconds.
    TimeLeft(u64),
    /// The game type identifier.
    GameType(u8),
    /// The rule type identifier (e.g., freestyle, renju).
    Rule(u8),
    /// Mouse cursor coordinates (optional for GUI-based players).
    Evaluate{
        x: i32,
        y: i32,
    },
    /// Path to a directory for persistent files (optional for local programs).
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

/// Represents a game end from a player perspective.
///
/// Used when the manager sends the `RESULT` command to a player.
#[derive(Debug, Clone)]
pub enum RelativeGameEnd {
    Draw,
    Win,
    Loose,
}

impl fmt::Display for RelativeGameEnd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match &self {
            RelativeGameEnd::Draw => write!(f, "0"),
            RelativeGameEnd::Win => write!(f, "1"),
            RelativeGameEnd::Loose => write!(f, "2"),
        }
    }
}
