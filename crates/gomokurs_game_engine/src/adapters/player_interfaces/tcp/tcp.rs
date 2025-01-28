use tokio::net::TcpStream;
use tokio::io::{ReadHalf, WriteHalf, split};
use thiserror::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Tcp {
    pub reader: Arc<Mutex<ReadHalf<TcpStream>>>,
    pub writer: Arc<Mutex<WriteHalf<TcpStream>>>,
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
        let stream = TcpStream::connect(cfg.player_address).await?;
        let (reader, writer) = split(stream);

        Ok(Self {
            reader: Arc::new(Mutex::new(reader)),
            writer: Arc::new(Mutex::new(writer)),
        })
    }
}

#[non_exhaustive]
pub struct ActionID;

impl ActionID {
    // Action that can be send from the manager to the player.

    pub const MANAGER_PROTOCOL_COMPATIBLE: u8   = 0x00;
    pub const MANAGER_START: u8                 = 0x01;
    pub const MANAGER_TURN: u8                  = 0x02;
    pub const MANAGER_BEGIN: u8                 = 0x03;
    pub const MANAGER_BOARD: u8                 = 0x04;
    pub const MANAGER_INFO: u8                  = 0x05;
    pub const MANAGER_END: u8                   = 0x06;
    pub const MANAGER_ABOUT: u8                 = 0x07;
    pub const MANAGER_UNKNOWN: u8               = 0x08;
    pub const MANAGER_ERROR: u8                 = 0x09;

    // Actions that can be send from the player to the manager.

    pub const PLAYER_PROTOCOL_VERSION: u8       = 0x0A;
    pub const PLAYER_READY: u8                  = 0x0B;
    pub const PLAYER_PLAY: u8                   = 0x0C;
    pub const PLAYER_PLAYER_DESCRIPTION: u8     = 0x0D;
    pub const PLAYER_UNKNOWN: u8                = 0x0E;
    pub const PLAYER_ERROR: u8                  = 0x0F;
    pub const PLAYER_MESSAGE: u8                = 0x10;
    pub const PLAYER_DEBUG: u8                  = 0x11;
    pub const PLAYER_SUGGESTION: u8             = 0x12;
}
