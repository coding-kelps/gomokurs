//! Define the game manager port models.

pub use crate::domain::board_state_manager::models::{PlayerColor, Position};
use crate::domain::game_manager::ports::PlayerNotifier;
use std::collections::HashMap;
use thiserror::Error;
use std::path::PathBuf;
use std::fmt;
use std::sync::Arc;
use crate::domain::game_manager::models::timer::Timer;
use tokio::time::Duration;

/// Represents a player managed by the game manager.
#[derive(Debug, Clone)]
pub struct Player<N>
where
    N: PlayerNotifier
{
    /// The color of the player (either black or white).
    pub color: PlayerColor,
    /// Does the player declared itself ready to play?
    pub ready: bool,
    /// The player metadata as key value pairs.
    pub description: Option<PlayerDescription>,
    /// The port by which the manager can notify the player program.
    pub notifier: Arc<N>,
    /// The timer of the player, when it runs out the player has lost.
    pub timer: Arc<Timer>,
}

impl<N> Player<N>
where
    N: PlayerNotifier
{
    /// Instantiate a new player.
    /// 
    /// # Arguments
    /// 
    /// * `color` - the color of the player that will be created.
    /// * `notifier` - An Arc to an adapter implementing the PlayerNotifier port
    /// by which the player program can be notified.
    /// * `turn_duration` - The duration of the player turn timer.
    /// * `match_duration` - The duration of the player match timer.
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
/// A player action.
/// 
/// Define the possible action that a player can send to the game manager.
/// For more details, see the
/// [`Gomocup Protocol`](https://plastovicka.github.io/protocl2en.htm).
/// It should be noted that a player action send by the player can be
/// wrong.
#[derive(Debug, Clone)]
pub enum PlayerAction {
    /// A response from the player to declare he's ready.
    Ok,
    /// A move made by the player to play a stone on the board
    /// at the given position.
    Play(Position),
    /// A description of the player metadata.
    Description(PlayerDescription),
    /// An error response from the player to indicates it didn't
    /// recognize the last action it received from the manager. 
    Unknown(String),
    /// An error response from the player to indicates it didn't
    /// recognize the last action it received from the manager. 
    Error(String),
    /// A message from the player to the other player.
    Message(String),
    /// A message sent by the player for debugging purpose.
    Debug(String),
    /// A move suggested by the player to the manager.
    Suggestion(Position),
}

/// The status of a cell from a player perspective.
/// 
/// It is only used by the manager to send the BOARD
/// command to a player.
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

/// A game turn from a player perspective.
/// 
/// It is only used by the manager to send the BOARD
/// command to a player.
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

/// A player metadata as a list of key value pairs.
#[derive(Debug, Clone)]
pub struct PlayerDescription {
    pub info: HashMap<String, String>,
}

/// An information that can be sent by the manager to a player.
#[derive(Debug, Clone)]
pub enum Information {
    /// The time limit of a player for each move as milliseconds.
    TimeoutTurn(u64),
    /// The time limit of a player for the whole match as milliseconds.
    TimeoutMatch(u64),
    /// The memory limit of a player program in bytes (optionaly used for local
    /// player program).
    MaxMemory(u64),
    /// The remaining time limit of a player for the whole match as
    /// milliseconds.
    TimeLeft(u64),
    /// The type of the game.
    GameType(u8),
    /// The type of rule followed for the game (e.g., freestyle, renju).
    Rule(u8),
    /// An coordinates representing the current position of a player mouse
    /// cursor (optionaly used for real player GUI).
    Evaluate{
        x: i32,
        y: i32,
    },
    /// The path to a local directory for players persistent files (optionaly
    /// used for local player program).
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

/// An error returned by a game manager service.
#[derive(Debug, Error)]
pub enum Error {
    /// An error returned by one of the two players notifier.
    #[error("failed to notify `{color}`: `{error}`")]
    NotifyError{
        error: NotifyError,
        color: PlayerColor,
    },
    /// An implementation-specific error
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// An error returned by a player notifier.
#[derive(Debug, Error)]
pub enum NotifyError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
