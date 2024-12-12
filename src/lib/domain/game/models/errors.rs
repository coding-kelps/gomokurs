use thiserror::Error;

// Service's errors
#[derive(Debug, Error)]
pub enum PlayError {
    #[error("channel abruptly close")]
    ActionsChannelAbruptlyClose,
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
pub enum NotifyStartError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum NotifyTurnError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum NotifyBeginError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum NotifyBoardError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum NotifyInfoError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum NotifyEndError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum NotifyAboutError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum NotifyUnknownError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum NotifyErrorError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
