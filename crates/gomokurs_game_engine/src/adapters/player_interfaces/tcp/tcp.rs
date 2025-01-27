use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use thiserror::Error;

pub struct Tcp {
    black_player_client: TcpStream,
    white_player_client: TcpStream,
}

pub struct CreateTcpInterfaceConfiguration {
    pub black_player_address: String,
    pub white_player_address: String,
}

#[derive(Debug, Error)]
pub enum CreateTcpInterfaceError {
    /// An error occurred when spawning the AI binary as a subprocess.
    #[error("create subprocess error: `{0}`")]
    CreateSubprocessError(#[from] tokio::io::Error),
}

impl Tcp {
    pub async fn new(cfg: CreateTcpInterfaceConfiguration) -> Result<Self, CreateTcpInterfaceError> {
        Ok(Self {
            black_player_client: TcpStream::connect(cfg.black_player_address).await?,
            white_player_client: TcpStream::connect(cfg.white_player_address).await?,
        })
    }
}
