use crate::domain::game::ports::GameService;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::io;
use thiserror::Error;

#[allow(dead_code)]
struct AppState<GS: GameService> {
    game_service: GS,
}

pub struct NamedPipe {
    path: PathBuf,
}

#[derive(Debug, Error)]
pub enum CreateNamedPipeError {
    #[error("no corresponding pipe at path: `{0}`")]
    NoPipe(String),
    #[error("error opening pipe for reading")]
    OpeningPipeError(#[from] io::Error),
}

impl NamedPipe {
    pub fn new(
        pipe_path: &str,
    ) -> Result<Self, CreateNamedPipeError> {
        if !Path::new(pipe_path).exists() {
            return Err(CreateNamedPipeError::NoPipe(pipe_path.into()));
        }

        let _pipe = match OpenOptions::new().read(true).open(pipe_path) {
            Ok(p) => p,
            Err(e) => {
                return Err(CreateNamedPipeError::OpeningPipeError(e));
            }
        };

        Ok(Self {
            path: PathBuf::from(pipe_path),
        })
    }
}

impl Drop for NamedPipe {
    fn drop(&mut self) {
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
