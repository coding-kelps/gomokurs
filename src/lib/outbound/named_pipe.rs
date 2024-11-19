use crate::domain::game::ports::PlayerClient;
use crate::domain::game::models::{RequestStartError, RequestTurnError, RequestBeginError, RequestBoardError, RequestInfoError, RequestEndError, RequestAboutError};
use tokio::fs::OpenOptions;
use tokio::process::Command;
use std::path::{Path, PathBuf};
use std::io;
use thiserror::Error;
use nix::unistd;
use nix::sys::stat;
use uuid::Uuid;

pub struct NamedPipe {
    fifo_in_path: PathBuf,
    fifo_out_path: PathBuf,
}

#[derive(Debug, Error)]
pub enum CreateNamedPipeError {
    #[error("failed to create pipe")]
    PipeCreationFailedError(#[from] nix::errno::Errno),
    #[error("error opening pipe for reading")]
    OpeningPipeError(#[from] io::Error),
}

impl NamedPipe {
    pub async fn new(binary: &Path) -> Result<Self, CreateNamedPipeError> {
        let uuid = Uuid::new_v4().to_string();
        let tmp_dir = format!("/tmp/gomokurs/{}/", uuid);
        let fifo_in_path = PathBuf::from(format!("{}/in", &tmp_dir));
        let fifo_out_path = PathBuf::from(format!("{}/out", &tmp_dir));

        for &fifo in &[&fifo_in_path, &fifo_out_path] {
            match unistd::mkfifo(fifo, stat::Mode::S_IRWXU) {
                Ok(_) => {},
                Err(e) => return Err(e.into()),
            }
        }

        let fifo_in = OpenOptions::new().read(true).open(&fifo_in_path).await?.into_std().await;
        let fifo_out = OpenOptions::new().write(true).open(&fifo_out_path).await?.into_std().await;

        let _ = Command::new(binary)
            .stdin(fifo_in)
            .stdout(fifo_out)
            .spawn()?;

        Ok(Self {
            fifo_in_path,
            fifo_out_path,
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

impl Drop for NamedPipe {
    fn drop(&mut self) {
        for &fifo in &[&self.fifo_in_path, &self.fifo_out_path] {
            match std::fs::remove_file(fifo) {
                Ok(_) => (),
                Err(e) => eprintln!("Error removing named pipe: {}", e),
            }
        }
    }
}
