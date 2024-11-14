use thiserror::Error;
use super::{CellStatus, Player, Position};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InitBoardRequest {
    size: Option<u8>,
}

#[derive(Debug, Error)]
pub enum InitBoardError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum GetBoardError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlayTurnRequest {
    position: Position,
    player: Player,
}

#[derive(Debug, Error)]
pub enum PlayTurnError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegisterTurnRequest {
    position: Position,
    status: CellStatus,
}

#[derive(Debug, Error)]
pub enum RegisterTurnError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

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
