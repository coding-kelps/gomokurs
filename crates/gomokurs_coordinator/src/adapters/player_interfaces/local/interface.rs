use tokio::process::{Command, Child, ChildStdin, ChildStdout};
use tokio::io::{AsyncBufReadExt, BufReader, Lines, BufWriter, AsyncWriteExt};
use crate::domain::coordinator::ports::PlayerInterface;
use crate::domain::coordinator::models::*;
use crate::adapters::player_interfaces::local::parsers::parse_input;
use tokio::sync::mpsc::Sender;
use anyhow::anyhow;
use std::process::Stdio;
use std::path::Path;
use thiserror::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Represents a local player interface for running a Gomoku AI binary as a
/// subprocess managed by the Gomoku manager.
pub struct LocalPlayerInterface {
    /// The child process representing the AI binary.
    _child: Child,
    /// The buffered reader for the AI process's standard output.
    pub reader: Arc<Mutex<Lines<BufReader<ChildStdout>>>>,
    /// The buffered writer for the AI process's standard input.
    pub writer: Arc<Mutex<BufWriter<ChildStdin>>>,
}

/// An error type for issues encountered when creating the local player
/// interface.
#[derive(Debug, Error)]
pub enum CreateLocalPlayerInterfaceError {
    /// An error occurred when spawning the AI binary as a subprocess.
    #[error("create subprocess error: `{0}`")]
    CreateSubprocessError(#[from] tokio::io::Error),
}

impl LocalPlayerInterface {
    /// Creates a new instance of the local player interface by launching the AI
    /// binary.
    /// 
    /// # Arguments
    ///
    /// * `binary` - The path to the AI binary to be executed as a subprocess.
    pub async fn new(binary: &Path, args: Vec<String>) -> Result<Self, CreateLocalPlayerInterfaceError> {
        let mut child = Command::new(binary)
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;

        // TODOs: handle stdout and stdin errors
        let stdout = child.stdout.take().expect("");
        let stdin = child.stdin.take().expect("");
        
        Ok(Self {
            _child: child,
            reader: Arc::new(Mutex::new(BufReader::new(stdout).lines())),
            writer: Arc::new(Mutex::new(BufWriter::new(stdin))),
        })
    }
}

impl PlayerInterface for LocalPlayerInterface {
    async fn listen(
        &self,
        player: PlayerColor,
        tx: Sender<(PlayerColor, PlayerAction)>,
    ) -> Result<(), ListenError> {
        loop {
            let line = {
                let mut locked_reader = self.reader.lock().await;
                
                locked_reader.next_line()
                    .await
                    .map_err(|e| anyhow!(e))?
                    .expect("self.reader.next_line() results is None")
            };

            match parse_input(line) {
                Ok(action) => {
                    tx.send((player, action)).await.expect("failed to send to channel");
                },
                Err(e) => {
                    println!("error at convertion {:?}", e);
                }
            };
        }
    }

    async fn notify_start(
        &self,
        size: u8,
    ) -> Result<(), NotifyError>
    {
        let mut writer = self.writer.lock().await;

        writer
            .write_all(format!("START {}\n", size).as_bytes())
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        writer
            .flush()
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    async fn notify_restart(
        &self
    ) -> Result<(), NotifyError> {
        let mut writer = self.writer.lock().await;

        writer
            .write_all(b"RESTART\n")
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        writer
            .flush()
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    async fn notify_turn(
        &self,
        position: Position,
    ) -> Result<(), NotifyError>
    {
        let mut writer = self.writer.lock().await;

        writer
            .write_all(format!("TURN {}\n", position).as_bytes())
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        writer
            .flush()
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    async fn notify_begin(
        &self,
    ) -> Result<(), NotifyError>
    {
        let mut writer = self.writer.lock().await;

        writer.write_all(b"BEGIN\n")
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        writer.flush()
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    async fn notify_board(
        &self,
        turns: Vec<RelativeTurn>,
    ) -> Result<(), NotifyError>
    {
        let mut writer = self.writer.lock().await;

        writer.write_all(b"BOARD\n")
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        for turn in turns {
            writer.write_all(format!("{}\n", turn).as_bytes())
                .await
                .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;
        }

        writer.write_all(b"DONE\n")
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    async fn notify_info(
        &self,
        info: Information,
    ) -> Result<(), NotifyError>
    {
        let mut writer = self.writer.lock().await;

        writer
            .write_all(format!("INFO {}\n", info).as_bytes())
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        writer
            .flush()
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    async fn notify_result(
            &self,
            result: RelativeGameEnd,
        ) -> Result<(), NotifyError> {
            let mut writer = self.writer.lock().await;



            writer
                .write_all(format!("RESULT {}\n", result).as_bytes())
                .await
                .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;
    
            writer
                .flush()
                .await
                .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;
    
            Ok(())
    }

    async fn notify_end(
        &self,
    ) -> Result<(), NotifyError>
    {
        let mut writer = self.writer.lock().await;

        let _ = writer.write_all(b"END");

        Ok(())
    }

    async fn notify_about(
        &self,
    ) -> Result<(), NotifyError>
    {
        let mut writer = self.writer.lock().await;

        let _ = writer.write_all(b"ABOUT");

        Ok(())
    }

    async fn notify_unknown(
        &self,
        content: &str,
    ) -> Result<(), NotifyError>
    {
        let mut writer = self.writer.lock().await;

        let _ = writer.write_all(format!("UNKNOWN {}", content).as_bytes());

        Ok(())
    }

    async fn notify_error(
        &self,
        content: &str,
    ) -> Result<(), NotifyError>
    {
        let mut writer = self.writer.lock().await;

        let _ = writer.write_all(format!("ERROR {}", content).as_bytes());

        Ok(())
    }
}

