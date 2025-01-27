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
    pub const START: u8                 = 0x01;
    pub const TURN: u8                  = 0x02;
    pub const BEGIN: u8                 = 0x03;
    pub const BOARD: u8                 = 0x04;
    pub const BOARD_TURN: u8            = 0x05;
    pub const BOARD_END: u8             = 0x06;
    pub const INFO: u8                  = 0x07;
    pub const END: u8                   = 0x08;
    pub const ABOUT: u8                 = 0x09;
    pub const READY: u8                 = 0x0A;
    pub const PLAY: u8                  = 0x0B;
    pub const PLAYER_DESCRIPTION: u8    = 0x0C;
    pub const UNKNOWN: u8               = 0x0D;
    pub const ERROR: u8                 = 0x0E;
    pub const MESSAGE: u8               = 0x0F;
    pub const DEBUG: u8                 = 0x10;
    pub const SUGGESTION: u8            = 0x11;
}
