//! Define the models of the player interfaces manager service.

pub use crate::domain::board_state_manager::models::{PlayerColor, Position};
pub use crate::domain::game_manager::models::Error as GameManagerError;
use tokio::task::JoinError;
use thiserror::Error;

/// An error returned by a player interfaces manager.
#[derive(Debug, Error)]
pub enum Error {
    /// An error returned when the channel used to coordinates received player
    /// actions unexpectedly closed.
    #[error("actions' channel abruptly closed ")]
    ChannelClosed,
    /// An error returned by the listeners join set.
    #[error("listeners join set error: `{0}`")]
    JoinError(#[from] JoinError),
    /// An error returned by a player listener.
    #[error("listener error: `{0}`")]
    ListenError(#[from] ListenError), 
    /// An error returned by the game manager.
    #[error("game error: `{0}`")]
    GameError(#[from] GameManagerError),
    /// An implementation specific error.
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// An Error returned by a player listener.
#[derive(Debug, Error)]
pub enum ListenError {
    /// An implementation specific error. 
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}