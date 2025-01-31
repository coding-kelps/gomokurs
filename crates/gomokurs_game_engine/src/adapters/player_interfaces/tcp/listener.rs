//! This module contains the implementation of the PlayerListener port for the
//! tcp player interface.

use crate::adapters::player_interfaces::tcp::tcp::{Tcp, ActionID};
use crate::domain::player_interfaces_manager::ports::PlayerListener;
use crate::domain::player_interfaces_manager::models::ListenError;
use crate::domain::game_manager::models::{PlayerColor, PlayerAction, Position, PlayerDescription};
use tokio::sync::mpsc::Sender;
use tokio::io::{self, AsyncReadExt};
use std::collections::HashMap;
use anyhow::anyhow;

impl PlayerListener for Tcp {
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
                ActionID::PLAYER_PLAYER_DESCRIPTION => self.player_description_handler()
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
}

impl Tcp {
    async fn ready_handler(
        &self,
    ) -> Result<PlayerAction, io::Error> {
        Ok(PlayerAction::Ok)
    }

    async fn play_handler(
        &self,
    ) -> Result<PlayerAction, io::Error> {
        let mut reader = self.reader.lock().await;

        let mut buf = [0u8; 2];

        reader.read_exact(&mut buf).await?;

        Ok(PlayerAction::Play(Position::new(buf[0], buf[1])))
    }

    async fn player_description_handler(
        &self,
    ) -> Result<PlayerAction, io::Error> {
        let info = HashMap::new();

        Ok(PlayerAction::Description(PlayerDescription{
            info: info,
        }))
    }

    async fn unknown_handler(
        &self,
    ) -> Result<PlayerAction, io::Error> {
        Ok(PlayerAction::Unknown(String::from("")))
    }

    async fn error_handler(
        &self,
    ) -> Result<PlayerAction, io::Error> {
        let mut reader = self.reader.lock().await;

        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf).await?;
        let payload_size = u32::from_be_bytes(buf) as usize;

        let mut buf = vec![0u8; payload_size];
        reader.read_exact(&mut buf).await?;
        let msg = String::from_utf8(buf).expect("from utf-8 error");

        Ok(PlayerAction::Error(msg))
    }

    async fn message_handler(
        &self,
    ) -> Result<PlayerAction, io::Error> {
        let mut reader = self.reader.lock().await;

        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf).await?;
        let payload_size = u32::from_be_bytes(buf) as usize;

        let mut buf = vec![0u8; payload_size];
        reader.read_exact(&mut buf).await?;
        let msg = String::from_utf8(buf).expect("from utf-8 error");

        Ok(PlayerAction::Message(msg))
    }

    async fn debug_handler(
        &self,
    ) -> Result<PlayerAction, io::Error> {
        let mut reader = self.reader.lock().await;

        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf).await?;
        let payload_size = u32::from_be_bytes(buf) as usize;

        let mut buf = vec![0u8; payload_size];
        reader.read_exact(&mut buf).await?;
        let msg = String::from_utf8(buf).expect("from utf-8 error");

        Ok(PlayerAction::Debug(msg))
    }

    async fn suggestion_handler(
        &self,
    ) -> Result<PlayerAction, io::Error> {
        let mut reader = self.reader.lock().await;

        let mut buf = [0u8; 2];

        reader.read_exact(&mut buf).await?;

        Ok(PlayerAction::Suggestion(Position::new(buf[0], buf[1])))
    }
}
