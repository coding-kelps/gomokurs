use thiserror::Error;
use super::{CellStatus, Player, Position};


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



// Repo's Requests
#[derive(Debug, Error)]
pub enum GetBoardError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegisterTurnRequest {
    pub position: Position,
    pub status: CellStatus,
}

#[derive(Debug, Error)]
pub enum RegisterTurnError {
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
