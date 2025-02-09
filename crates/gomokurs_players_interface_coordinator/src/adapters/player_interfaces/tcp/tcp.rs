use tokio::net::TcpStream;
use tokio::io::{ReadHalf, WriteHalf, split};
use thiserror::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const PROTOCOL_VERSION: &str            = "0.1.0";

#[non_exhaustive]
pub struct ActionID;

impl ActionID {
    // Action that can be send from the manager to the player.

    pub const MANAGER_PROTOCOL_COMPATIBLE: u8   = 0x00;
    pub const MANAGER_START: u8                 = 0x01;
    pub const MANAGER_RESTART: u8               = 0x02;
    pub const MANAGER_TURN: u8                  = 0x03;
    pub const MANAGER_BEGIN: u8                 = 0x04;
    pub const MANAGER_BOARD: u8                 = 0x05;
    pub const MANAGER_INFO: u8                  = 0x06;
    pub const MANAGER_RESULT: u8                = 0x07;
    pub const MANAGER_END: u8                   = 0x08;
    pub const MANAGER_ABOUT: u8                 = 0x09;
    pub const MANAGER_UNKNOWN: u8               = 0x0A;
    pub const MANAGER_ERROR: u8                 = 0x0B;

    // Actions that can be send from the player to the manager.

    pub const PLAYER_PROTOCOL_VERSION: u8       = 0x0C;
    pub const PLAYER_READY: u8                  = 0x0D;
    pub const PLAYER_PLAY: u8                   = 0x0E;
    pub const PLAYER_PLAYER_DESCRIPTION: u8     = 0x0F;
    pub const PLAYER_UNKNOWN: u8                = 0x10;
    pub const PLAYER_ERROR: u8                  = 0x11;
    pub const PLAYER_MESSAGE: u8                = 0x12;
    pub const PLAYER_DEBUG: u8                  = 0x13;
    pub const PLAYER_SUGGESTION: u8             = 0x14;
}

pub struct Tcp {
    pub reader: Arc<Mutex<ReadHalf<TcpStream>>>,
    pub writer: Arc<Mutex<WriteHalf<TcpStream>>>,
}

pub struct CreateTcpInterfaceConfiguration {
    pub stream: TcpStream
}

#[derive(Debug, Error)]
pub enum CreateTcpInterfaceError {
    #[error("create tcp interface error: `{0}`")]
    CreateClientError(#[from] tokio::io::Error),
    #[error("manager tcp player interface version `{manager_version}` is incompatible with player manager interface version `{player_version}`")]
    IncompatibleProtocolError{
        manager_version:    String,
        player_version:     String,
    },
}

impl Tcp {
    pub async fn new(cfg: CreateTcpInterfaceConfiguration) -> Result<Self, CreateTcpInterfaceError> {
        let (mut reader, mut writer) = split(cfg.stream);

        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf).await?;

        if buf[0] == ActionID::PLAYER_PROTOCOL_VERSION {
            let mut buf = [0u8; 4];
            reader.read_exact(&mut buf).await?;
            let payload_size = u32::from_be_bytes(buf) as usize;

            let mut buf = vec![0u8; payload_size];
            reader.read_exact(&mut buf).await?;
            let player_version = String::from_utf8(buf).expect("from utf-8 error");

            if player_version == PROTOCOL_VERSION {
                writer
                    .write_all(&[ActionID::MANAGER_PROTOCOL_COMPATIBLE])
                    .await?;
            } else {
                let err = CreateTcpInterfaceError::IncompatibleProtocolError {
                    manager_version: String::from(PROTOCOL_VERSION),
                    player_version: player_version
                };
                let err_msg = format!("{}", err);
                let bytes_error_msg = err_msg.as_bytes();
                let bytes_error_msg_len: &[u8] = &(bytes_error_msg.len() as u32).to_be_bytes();

                let data = [&[ActionID::MANAGER_ERROR], bytes_error_msg_len, bytes_error_msg].concat();

                writer
                    .write_all(&data)
                    .await?;

                return Err(err)
            }
        } else {
            let error_msg = format!("unexpected version");
            let bytes_error_msg = error_msg.as_bytes();
            let bytes_error_msg_len: &[u8] = &(bytes_error_msg.len() as u32).to_be_bytes();

            let data = [&[ActionID::MANAGER_ERROR], bytes_error_msg_len, bytes_error_msg].concat();

            writer
                .write_all(&data)
                .await?;
        }

        Ok(Self {
            reader: Arc::new(Mutex::new(reader)),
            writer: Arc::new(Mutex::new(writer)),
        })
    }
}
