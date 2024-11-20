use crate::domain::game::ports::PlayerClient;
use crate::domain::game::models::{RequestStartError, RequestTurnError, RequestBeginError, RequestBoardError, RequestInfoError, RequestEndError, RequestAboutError};
use tokio::process::{Command, Child, ChildStdin, ChildStdout};
use tokio::io::{AsyncBufReadExt, BufReader, Lines, BufWriter};
use std::process::Stdio;
use std::path::Path;
use thiserror::Error;

pub struct NamedPipe {
    _child: Child,
    pub reader: Lines<BufReader<ChildStdout>>,
    pub writer: BufWriter<ChildStdin>,
}

#[derive(Debug, Error)]
pub enum CreateNamedPipeError {
    #[error("create subprocess error: `{0}`")]
    CreateSubprocessError(#[from] tokio::io::Error),
}

impl NamedPipe {
    pub async fn new(binary: &Path) -> Result<Self, CreateNamedPipeError> {
        let mut child = Command::new(binary)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;

        let stdout = child.stdout.take().expect("");
        let stdin = child.stdin.take().expect("");

        let reader = BufReader::new(stdout).lines();
        let writer = BufWriter::new(stdin);
        
        Ok(Self {
            _child: child,
            reader,
            writer,
        })
    }
}

impl PlayerClient for NamedPipe {
    fn request_start(
        &self,
    ) -> Result<(), RequestStartError>
    {
        Ok(())
    }

    fn request_turn(
        &self,
    ) -> Result<(), RequestTurnError>
    {
        Ok(())
    }

    fn request_begin(
        &self,
    ) -> Result<(), RequestBeginError>
    {
        Ok(())
    }

    fn request_board(
        &self,
    ) -> Result<(), RequestBoardError>
    {
        Ok(())
    }

    fn request_info(
        &self,
    ) -> Result<(), RequestInfoError>
    {
        Ok(())
    }

    fn request_end(
        &self,
    ) -> Result<(), RequestEndError>
    {
        Ok(())
    }

    fn request_about    (
        &self,
    ) -> Result<(), RequestAboutError>
    {
        Ok(())
    }
}
