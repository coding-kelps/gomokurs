use crate::domain::game::models::Position;
use crate::domain::game::ports::GameService;
use tokio::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::io;
use tokio::io::{AsyncBufReadExt, BufReader};
use thiserror::Error;
use nix::unistd;
use nix::sys::stat;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use regex::Regex;

#[allow(dead_code)]
struct AppState<GS: GameService> {
    game_service: GS,
}

pub struct NamedPipe {
    path: PathBuf,
    stop_signal: Arc<AtomicBool>,
    _handle: tokio::task::JoinHandle<()>,
}

#[derive(Debug, Error)]
pub enum CreateNamedPipeError {
    #[error("failed to create pipe")]
    PipeCreationFailedError(#[from] nix::errno::Errno),
    #[error("error opening pipe for reading")]
    OpeningPipeError(#[from] io::Error),
}

#[derive(Debug, Error)]
enum ParseCommandError {
    #[error("failed to regular expression")]
    RegexCreationError(#[from] regex::Error),
    #[error("failed to parse number")]
    NumberParsingError(#[from]  std::num::ParseIntError),
    #[error("unknown command: `{0}`")]
    UnknownCommand(String)
}

async fn parse_command(
    command: &str,
) -> Result<(), ParseCommandError> {
    let re = Regex::new(r"^TURN (\d+),(\d+)$")?;

    match re.captures(command) {
        Some(caps) => {
            let position = Position::new(
                caps[1].parse::<u8>()?,
                caps[2].parse::<u8>()?);

            // Call GameService::PlayTurn

            println!("Play Turn at position: {}", position);
        }
        None => return Err(ParseCommandError::UnknownCommand(command.into())),
    }

    Ok(())
}

impl NamedPipe {
    pub async fn new(
        pipe_path: &str,
    ) -> Result<Self, CreateNamedPipeError> {
        if !Path::new(pipe_path).exists() {
            match unistd::mkfifo(pipe_path, stat::Mode::S_IRWXU) {
                Ok(_) => {},
                Err(e) => return Err(e.into()),
            }
        }
        
        let stop_signal = Arc::new(AtomicBool::new(false));
        let stop_signal_clone = Arc::clone(&stop_signal);
        let pipe = match OpenOptions::new().read(true).open(pipe_path).await {
            Ok(p) => p,
            Err(e) => {
                return Err(e.into());
            }
        };

        let handle = tokio::spawn(async move {
            let reader = BufReader::new(pipe);
            let mut lines = reader.lines();

            while !stop_signal_clone.load(Ordering::Relaxed) {
                match lines.next_line().await {
                    Ok(res) => {
                        if let Some(line) = res {
                            if let Err(e) = parse_command(&line).await {
                                eprintln!("Named Pipe error: {}", e);
                            }
                        }
                    }
                    Err(e) => eprintln!("error: {}", e),
                }
            }
        });

        Ok(Self {
            path: PathBuf::from(pipe_path),
            stop_signal: stop_signal,
            _handle: handle,
        })
    }
}

impl Drop for NamedPipe {
    fn drop(&mut self) {
        self.stop_signal.store(true, Ordering::Relaxed);

        if self.path.exists() {
            match std::fs::remove_file(self.path.clone()) {
                Ok(_) => (),
                Err(e) => eprintln!("Error removing named pipe: {}", e),
            }
        } else {
            println!("Named pipe does not exist or was already removed.");
        }
    }
}
