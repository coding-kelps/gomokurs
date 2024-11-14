use crate::domain::game::ports::GameService;
use std::fs::OpenOptions;
use std::path::Path;
use thiserror::Error;

struct AppState<GS: GameService> {
    game_service: GS,
}

pub struct NamedPipe {
    path: Path,
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum CreateNamedPipeError {
    #[error("path is unavailable for named pipe creation")]
    UnavailablePath,
}

impl NamedPipe {
    pub fn new(
        pipe_path: &str,
        game_service: impl GameService,
    ) -> Result<Self, CreateNamedPipeError> {
        if !Path::new(pipe_path).exists() {
            Err(CreateNamedPipeError::UnavailablePath)
        }

        let pipe = match OpenOptions::new().read(true).open(pipe_path) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Error opening pipe for reading: {}", e);
                return;
            }
        };

        Ok(Self {
            path: pipe_path,
        })
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
