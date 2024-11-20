use crate::domain::game::ports::PlayerClient;
use crate::domain::game::models::{Position, RequestStartError, RequestTurnError, RequestBeginError, RequestBoardError, RequestInfoError, RequestEndError, RequestAboutError};
use tokio::process::{Command, Child, ChildStdin, ChildStdout};
use tokio::io::{AsyncBufReadExt, BufReader, Lines, AsyncWriteExt, BufWriter};
use std::num::ParseIntError;
use std::process::Stdio;
use std::path::Path;
use thiserror::Error;
use regex::Regex;
use anyhow::anyhow;

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

#[derive(Debug, Error)]
enum ParsePlayerMoveError {
    #[error("regular expression failed to compile")]
    InvalidRegex(#[from] regex::Error),
    #[error("player move format is invalid")]
    InvalidFormat,
    #[error("move's coordinates are invalid: `{0}`")]
    InvalidCoordinates(#[from] ParseIntError),
}

fn parse_player_move(
    res: &str
) -> Result<Position, ParsePlayerMoveError> {
    let re = Regex::new(r"^(\d+),(\d+)$")?;

    match re.captures(res) {
        Some(caps) => {
            let x = caps[1].parse::<u8>()?;
            let y = caps[2].parse::<u8>()?;

            Ok(Position::new(x, y))
        }
        None => Err(ParsePlayerMoveError::InvalidFormat),
    }
}

impl PlayerClient for NamedPipe {
    async fn request_start(
        &mut self,
        size: u8,
    ) -> Result<(), RequestStartError>
    {
        Ok(())
    }

    async fn request_turn(
        &mut self,
        position: Position,
    ) -> Result<Position, RequestTurnError>
    {
        Ok(Position::new(0, 0))
    }

    async fn request_begin(
        &mut self,
    ) -> Result<Position, RequestBeginError>
    {
        self.writer
            .write_all(b"BEGIN\n")
            .await
            .map_err(|e| RequestBeginError::Unknown(anyhow!(e)))?;

        self.writer
            .flush()
            .await
            .map_err(|e| RequestBeginError::Unknown(anyhow!(e)))?;

        let line = self.reader.next_line()
            .await
            .map_err(|e| RequestBeginError::Unknown(anyhow!(e)))?
            .expect("self.reader.next_line() results is None");

        match parse_player_move(&line) {
            Ok(p) => {
                return Ok(p)
            },
            Err(e) => Err(RequestBeginError::Unknown(e.into())),
        }
    }

    async fn request_board(
        &mut self,
        positions: Vec<Position>,
    ) -> Result<(), RequestBoardError>
    {
        Ok(())
    }

    async fn request_info(
        &mut self,
    ) -> Result<(), RequestInfoError>
    {
        Ok(())
    }

    async fn request_end(
        &mut self,
    ) -> Result<(), RequestEndError>
    {
        let _ = self.writer.write_all(b"END");

        Ok(())
    }

    async fn request_about    (
        &mut self,
    ) -> Result<(), RequestAboutError>
    {
        let _ = self.writer.write_all(b"ABOUT");

        Ok(())
    }
}
