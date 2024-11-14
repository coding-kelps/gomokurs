use crate::domain::game::ports::GameService;
use std::fs::OpenOptions;
use std::path::Path;
use std::io;
use thiserror::Error;

struct AppState<GS: GameService> {
    game_service: GS,
}

pub struct NamedPipe {
    path: Path,
}

#[derive(Debug, Error)]
pub enum CreateNamedPipeError {
    #[error("path is unavailable for named pipe creation")]
    UnavailablePath,
    #[error("error opening pipe for reading")]
    OpeningPipeError(#[from] io::Error),
}

impl NamedPipe {
    pub fn new(
        pipe_path: &str,
        game_service: impl GameService,
    ) -> Self {
        if !Path::new(pipe_path).exists() {
            return Err(CreateNamedPipeError::UnavailablePath);
        }

        let pipe = match OpenOptions::new().read(true).open(pipe_path) {
            Ok(p) => p,
            Err(e) => {
                return Err(CreateNamedPipeError::OpeningPipeError(e));
            }
        };

        Self {
            path: pipe_path,
        }
    }
}

impl Drop for NamedPipe {
    fn drop(&mut self) {
        if self.path.exists() {
            match std::fs::remove_file(self.path) {
                Ok(_) => (),
                Err(e) => eprintln!("Error removing named pipe: {}", e),
            }
        } else {
            println!("Named pipe does not exist or was already removed.");
        }
    }
}
