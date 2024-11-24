use crate::domain::game::ports::PlayerClient;
use crate::domain::game::models::{Information, PlayerInformations, Position, RelativeTurn, RequestAboutError, RequestBeginError, RequestBoardError, RequestEndError, RequestInfoError, RequestStartError, RequestTurnError};
use tokio::process::{Command, Child, ChildStdin, ChildStdout};
use tokio::io::{AsyncBufReadExt, BufReader, Lines, AsyncWriteExt, BufWriter};
use std::process::Stdio;
use std::path::Path;
use thiserror::Error;
use crate::outbound::local_program::parsers::{parse_position, parse_player_informations};
use anyhow::anyhow;

pub struct LocalProgram {
    _child: Child,
    pub reader: Lines<BufReader<ChildStdout>>,
    pub writer: BufWriter<ChildStdin>,
}

#[derive(Debug, Error)]
pub enum CreateLocalProgramError {
    #[error("create subprocess error: `{0}`")]
    CreateSubprocessError(#[from] tokio::io::Error),
}

impl LocalProgram {
    pub async fn new(binary: &Path) -> Result<Self, CreateLocalProgramError> {
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

impl PlayerClient for LocalProgram {
    async fn request_start(
        &mut self,
        size: u8,
    ) -> Result<(), RequestStartError>
    {
        self.writer
            .write_all(format!("START {}\n", size).as_bytes())
            .await
            .map_err(|e| RequestStartError::Unknown(anyhow!(e)))?;

        self.writer
            .flush()
            .await
            .map_err(|e| RequestStartError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    async fn request_turn(
        &mut self,
        position: Position,
    ) -> Result<Position, RequestTurnError>
    {
        self.writer
            .write_all(format!("TURN {}\n", position).as_bytes())
            .await
            .map_err(|e| RequestTurnError::Unknown(anyhow!(e)))?;

        self.writer
            .flush()
            .await
            .map_err(|e| RequestTurnError::Unknown(anyhow!(e)))?;

        let line = self.reader.next_line()
            .await
            .map_err(|e| RequestTurnError::Unknown(anyhow!(e)))?
            .expect("self.reader.next_line() results is None");

        match parse_position(&line) {
            Ok(p) => {
                return Ok(p)
            },
            Err(e) => Err(RequestTurnError::Unknown(e.into())),
        }
    }

    async fn request_begin(
        &mut self,
    ) -> Result<Position, RequestBeginError>
    {
        self.writer.write_all(b"BEGIN\n")
            .await
            .map_err(|e| RequestBeginError::Unknown(anyhow!(e)))?;

        self.writer.flush()
            .await
            .map_err(|e| RequestBeginError::Unknown(anyhow!(e)))?;

        let line = self.reader.next_line()
            .await
            .map_err(|e| RequestBeginError::Unknown(anyhow!(e)))?
            .expect("self.reader.next_line() results is None");

        match parse_position(&line) {
            Ok(p) => {
                return Ok(p)
            },
            Err(e) => Err(RequestBeginError::Unknown(e.into())),
        }
    }

    async fn request_board(
        &mut self,
        turns: Vec<RelativeTurn>,
    ) -> Result<Position, RequestBoardError>
    {
        self.writer.write_all(b"BOARD\n")
            .await
            .map_err(|e| RequestBoardError::Unknown(anyhow!(e)))?;

        for turn in turns {
            self.writer.write_all(format!("{}\n", turn).as_bytes())
                .await
                .map_err(|e| RequestBoardError::Unknown(anyhow!(e)))?;
        }

        self.writer.write_all(b"DONE\n")
            .await
            .map_err(|e| RequestBoardError::Unknown(anyhow!(e)))?;

        let line = self.reader.next_line()
            .await
            .map_err(|e| RequestBoardError::Unknown(anyhow!(e)))?
            .expect("self.reader.next_line() results is None");

        match parse_position(&line) {
            Ok(p) => {
                return Ok(p)
            },
            Err(e) => Err(RequestBoardError::Unknown(e.into())),
        }
    }

    async fn request_info(
        &mut self,
        info: Information,
    ) -> Result<(), RequestInfoError>
    {
        self.writer
            .write_all(format!("INFO {}\n", info).as_bytes())
            .await
            .map_err(|e| RequestInfoError::Unknown(anyhow!(e)))?;

        self.writer
            .flush()
            .await
            .map_err(|e| RequestInfoError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    async fn request_end(
        &mut self,
    ) -> Result<(), RequestEndError>
    {
        let _ = self.writer.write_all(b"END");

        Ok(())
    }

    async fn request_about(
        &mut self,
    ) -> Result<PlayerInformations, RequestAboutError>
    {
        let _ = self.writer.write_all(b"ABOUT");

        let line = self.reader.next_line()
            .await
            .map_err(|e| RequestAboutError::Unknown(anyhow!(e)))?
            .expect("self.reader.next_line() results is None");

        Ok(parse_player_informations(&line))
    }
}
