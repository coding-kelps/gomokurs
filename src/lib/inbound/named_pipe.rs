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
                            println!("received command: {}", line);
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
