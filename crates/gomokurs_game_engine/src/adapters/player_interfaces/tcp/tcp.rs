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

#[non_exhaustive]
pub struct CommandID;

impl CommandID {
    pub const START: u8 = 0x01;
    pub const TURN: u8 = 0x02;
    pub const BEGIN: u8 = 0x03;
    pub const BOARD: u8 = 0x04;
    pub const BOARD_TURN: u8 = 0x05;
    pub const BOARD_END: u8 = 0x06;
    pub const INFO: u8 = 0x07;
    pub const END: u8 = 0x08;
    pub const ABOUT: u8 = 0x09;
}
