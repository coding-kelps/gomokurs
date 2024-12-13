use thiserror::Error;

use super::PlayerColor;

// Service's errors
#[derive(Debug, Error)]
pub enum Error {
    #[error("channel abruptly close")]
    ActionsChannelAbruptlyClose,
    #[error("failed to notify `{player}`: `{error}`")]
    NotifyError{
        error: NotifyError,
        player: PlayerColor,
    },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

// PlayerClient's errors
#[derive(Debug, Error)]
pub enum ListenError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum NotifyError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
