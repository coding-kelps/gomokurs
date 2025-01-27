use tokio::net::TcpStream;
use thiserror::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Tcp {
    pub writer: Arc<Mutex<TcpStream>>,
}

pub struct CreateTcpInterfaceConfiguration {
    pub player_address: String,
}

#[derive(Debug, Error)]
pub enum CreateTcpInterfaceError {
    /// An error occurred when spawning the AI binary as a subprocess.
    #[error("create tcp client error: `{0}`")]
    CreateClientError(#[from] tokio::io::Error),
}

impl Tcp {
    pub async fn new(cfg: CreateTcpInterfaceConfiguration) -> Result<Self, CreateTcpInterfaceError> {
        Ok(Self {
            writer: Arc::new(Mutex::new(TcpStream::connect(cfg.player_address).await?)),
        })
    }
}
