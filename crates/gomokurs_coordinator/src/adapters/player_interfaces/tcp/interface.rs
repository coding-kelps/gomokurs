use tokio::net::TcpStream;
use tokio::io::{ReadHalf, WriteHalf, split};
use thiserror::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::adapters::player_interfaces::tcp::protocol::*;
use crate::domain::coordinator::models::*;
use crate::domain::coordinator::ports::PlayerInterface;
use tokio::sync::mpsc::Sender;
use anyhow::anyhow;

pub struct TcpPlayerInterface {
    pub reader: Arc<Mutex<ReadHalf<TcpStream>>>,
    pub writer: Arc<Mutex<WriteHalf<TcpStream>>>,
}

pub struct CreateTcpPlayerInterfaceConfiguration {
    pub stream: TcpStream
}

#[derive(Debug, Error)]
pub enum CreateTcpPlayerInterfaceError {
    #[error("create tcp interface error: `{0}`")]
    CreateClientError(#[from] tokio::io::Error),
    #[error("manager tcp player interface version `{manager_version}` is incompatible with player manager interface version `{player_version}`")]
    IncompatibleProtocolError{
        manager_version:    String,
        player_version:     String,
    },
}

impl TcpPlayerInterface {
    pub async fn new(cfg: CreateTcpPlayerInterfaceConfiguration) -> Result<Self, CreateTcpPlayerInterfaceError> {
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
                let err = CreateTcpPlayerInterfaceError::IncompatibleProtocolError {
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


impl PlayerInterface for TcpPlayerInterface {
    async fn listen(
        &self,
        player: PlayerColor,
        tx: Sender<(PlayerColor, PlayerAction)>,
    ) -> Result<(), ListenError> {
        let mut buf = vec![0u8; 1];

        loop {
            let mut reader = self.reader.lock().await;

            reader.read_exact(&mut buf)
                .await
                .map_err(|e| ListenError::Unknown(anyhow!(e)))?;

            std::mem::drop(reader);
            
            let action = match buf[0] {
                ActionID::PLAYER_READY => self.ready_handler()
                    .await
                    .map_err(|e| ListenError::Unknown(anyhow!(e)))?,
                ActionID::PLAYER_PLAY => self.play_handler()
                    .await
                    .map_err(|e| ListenError::Unknown(anyhow!(e)))?,
                ActionID::PLAYER_METADATA => self.player_description_handler()
                    .await
                    .map_err(|e| ListenError::Unknown(anyhow!(e)))?,
                ActionID::PLAYER_UNKNOWN => self.unknown_handler()
                    .await
                    .map_err(|e| ListenError::Unknown(anyhow!(e)))?,
                ActionID::PLAYER_ERROR => self.error_handler()
                    .await
                    .map_err(|e| ListenError::Unknown(anyhow!(e)))?,
                ActionID::PLAYER_MESSAGE => self.message_handler()
                    .await
                    .map_err(|e| ListenError::Unknown(anyhow!(e)))?,
                ActionID::PLAYER_DEBUG => self.debug_handler()
                    .await
                    .map_err(|e| ListenError::Unknown(anyhow!(e)))?,
                ActionID::PLAYER_SUGGESTION => self.suggestion_handler()
                    .await
                    .map_err(|e| ListenError::Unknown(anyhow!(e)))?,
                _ => continue,
            };

            tx.send((player, action)).await.expect("failed to send to channel");
        }
    }

    async fn notify_start(
        &self,
        size: u8,
    ) -> Result<(), NotifyError>
    {
        let mut writer = self.writer.lock().await;

        writer
            .write_all(&[ActionID::MANAGER_START, size])
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;
        
        Ok(())
    }

    async fn notify_restart(
        &self
    ) -> Result<(), NotifyError> {
        let mut writer = self.writer.lock().await;

        writer
            .write_all(&[ActionID::MANAGER_RESTART])
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;
        
        Ok(())
    }

    async fn notify_turn(
        &self,
        position: Position,
    ) -> Result<(), NotifyError>
    {
        let mut writer = self.writer.lock().await;

        writer
            .write_all(&[ActionID::MANAGER_TURN, position.x, position.y])
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    async fn notify_begin(
        &self,
    ) -> Result<(), NotifyError>
    {
        let mut writer = self.writer.lock().await;

        writer
            .write_all(&[ActionID::MANAGER_BEGIN])
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    async fn notify_board(
        &self,
        turns: Vec<RelativeTurn>,
    ) -> Result<(), NotifyError>
    {
        let mut writer = self.writer.lock().await;

        writer
            .write_all(&[ActionID::MANAGER_BOARD])
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        for turn in turns {
            let field = match turn.field {
                RelativeField::OwnStone => 0,
                RelativeField::OpponentStone => 1,
            };

            writer
                .write_all(&[turn.position.x, turn.position.y, field])
                .await
                .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;
        }

        Ok(())
    }

    async fn notify_info(
        &self,
        info: Information,
    ) -> Result<(), NotifyError>
    {
        let mut writer = self.writer.lock().await;

        let info_str = info.to_string();
        let bytes_info = info_str.as_bytes();
        let bytes_info_len: &[u8] = &(bytes_info.len() as u32).to_be_bytes();

        let data = [&[ActionID::MANAGER_INFO], bytes_info_len, bytes_info].concat();

        writer
            .write_all(&data)
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;
        
        Ok(())
    }

    async fn notify_result(
        &self,
        result: RelativeGameEnd,
    ) -> Result<(), NotifyError> {
        let mut writer = self.writer.lock().await;

        let result_as_byte = match result {
            RelativeGameEnd::Draw => 0,
            RelativeGameEnd::Win => 1,
            RelativeGameEnd::Loose => 2,
        };

        writer
            .write_all(&[ActionID::MANAGER_RESULT, result_as_byte])
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    async fn notify_end(
        &self,
    ) -> Result<(), NotifyError>
    {
        let mut writer = self.writer.lock().await;

        writer
            .write_all(&[ActionID::MANAGER_END])
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    async fn notify_about(
        &self,
    ) -> Result<(), NotifyError>
    {
        let mut writer = self.writer.lock().await;

        writer
            .write_all(&[ActionID::MANAGER_ABOUT])
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    #[allow(unused_variables)]
    async fn notify_unknown(
        &self,
        content: &str,
    ) -> Result<(), NotifyError> {
        Ok(())
    }

    #[allow(unused_variables)]
    async fn notify_error(
        &self,
        content: &str,
    ) -> Result<(), NotifyError>
    {
        Ok(())
    }
}

