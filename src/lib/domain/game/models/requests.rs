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


// Player Client's Requests
#[derive(Debug, Error)]
pub enum RequestStartError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum RequestTurnError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum RequestBeginError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum RequestBoardError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum RequestInfoError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum RequestEndError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum RequestAboutError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
