use thiserror::Error;

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
}

#[derive(Debug, Error)]
pub enum PlayTurnError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum RegisterTurnError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResetRequest {
    size: Option<u8>,
}

#[derive(Debug, Error)]
pub enum ResetBoardError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
