//! Models for the `PlayerInterfacesManager` service.
//!
//! This module defines errors and data types used within the
//! `PlayerInterfacesManager`, a domain service responsible for managing player
//! listeners and handling player actions.

use crate::domain::coordinator::models::actions::{PlayerColor, GameEngineError};
use tokio::task::JoinError;
use thiserror::Error;

/// Errors returned by the `PlayerInterfacesManager` service.
#[derive(Debug, Error)]
pub enum Error {
    /// Indicates that the channel coordinating received player actions was
    /// unexpectedly closed.
    #[error("actions' channel abruptly closed ")]
    ChannelClosed,
    /// An error occurred within the listeners' join set.
    #[error("listeners join set error: `{0}`")]
    JoinError(#[from] JoinError),
    /// An error was returned by a player listener.
    #[error("listener error: `{0}`")]
    ListenError(#[from] ListenError),
    /// Error encountered while notifying a player.
    #[error("failed to notify `{color}`: `{error}`")]
    NotifyError{
        error: NotifyError,
        color: PlayerColor,
    },
    /// An error propagated from the game manager.
    #[error("game error: `{0}`")]
    GameError(#[from] GameEngineError),
    /// For implementation-specific error.
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Errors returned by a player listener.
///
/// A `PlayerListener` is an adapter for interfacing with individual players.
#[derive(Debug, Error)]
pub enum ListenError {
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
