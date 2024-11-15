use thiserror::Error;
use super::{Player, Position};


// Service's Requests
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlayTurnRequest {
    pub player: Player,
    pub position: Position,
}

#[derive(Debug, Error)]
pub enum PlayTurnError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}


// Notifier's Requests
#[derive(Debug, Error)]
pub enum NotifyBeginError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum NotifyEndError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
