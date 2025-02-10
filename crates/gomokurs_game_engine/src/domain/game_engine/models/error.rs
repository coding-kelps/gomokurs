use thiserror::Error;
use crate::domain::game_engine::models::state::{PlayerColor, SetCellError};

/// Errors that may occur in the Game Manager service.
#[derive(Debug, Error)]
pub enum Error {
    /// Attempted to play out of turn.
    #[error("it is not `{0}` turn")]
    NotPlayerTurn(PlayerColor),
    /// An error occurred while setting a cell's status.
    #[error("set cell error: `{0}`")]
    SetCellError(#[from] SetCellError),
    /// For implementation-specific error.
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
