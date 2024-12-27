//! Player-related models for the Game Manager port.

pub use crate::domain::board_state_manager::models::{PlayerColor, Position};
use crate::domain::game_manager::ports::PlayerNotifier;
use std::collections::HashMap;
use thiserror::Error;
use std::path::PathBuf;
use std::fmt;
use std::sync::Arc;
use crate::domain::game_manager::models::timer::Timer;
use tokio::time::Duration;

/// Represents a player managed by the Game Manager.
#[derive(Debug, Clone)]
pub struct Player<N>
where
    N: PlayerNotifier
{
    /// The player's assigned color (either black or white).
    pub color: PlayerColor,
    /// Indicates if the player has declared readiness to play.
    pub ready: bool,
    /// Metadata about the player as key-value pairs.
    pub description: Option<PlayerDescription>,
    /// The notifier used to communicate with the player program.
    pub notifier: Arc<N>,
    /// The player's timers for turn and match durations.
    pub timer: Arc<Timer>,
}

impl<N> Player<N>
where
    N: PlayerNotifier
{
    /// Creates a new player.
    /// 
    /// # Arguments
    /// 
    /// * `color` - The player's assigned color.
    /// * `notifier` - An `Arc` pointing to an adapter implementing the `PlayerNotifier` port.
    /// * `turn_duration` - The time allocated for a single turn.
    /// * `match_duration` - The total time allocated for the match.
    pub fn new(
        color: PlayerColor,
        notifier: Arc<N>,
        turn_duration: Duration,
        match_duration: Duration,
    ) -> Self {
        Self {
            color,
            ready: false,
            description: None,
            notifier: notifier,
            timer: Arc::new(Timer::new(turn_duration, match_duration)),
        }
    }
}

/// Represents an action initiated by a player.
///
/// Actions may originate from the player program and can include invalid inputs.
/// For more information, refer to the
/// [`Gomocup Protocol`](https://plastovicka.github.io/protocl2en.htm).
#[derive(Debug, Clone)]
pub enum PlayerAction {
    /// The player declares readiness to play.
    Ok,
    /// The player makes a move at the specified board position.
    Play(Position),
    /// Provides metadata about the player.
    Description(PlayerDescription),
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

/// Metadata about a player as a collection of key-value pairs.
#[derive(Debug, Clone)]
pub struct PlayerDescription {
    pub info: HashMap<String, String>,
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

/// Errors that may occur in the Game Manager service.
#[derive(Debug, Error)]
pub enum Error {
    /// Error encountered while notifying a player.
    #[error("failed to notify `{color}`: `{error}`")]
    NotifyError{
        error: NotifyError,
        color: PlayerColor,
    },
    /// For implementation-specific error.
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Errors that may occur while notifying a player.
#[derive(Debug, Error)]
pub enum NotifyError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
