use crate::adapters::player_interfaces::tcp::TcpPlayerInterface;
use crate::domain::coordinator::models::*;
use std::collections::HashMap;
use tokio::io::AsyncReadExt;

impl TcpPlayerInterface {
    pub async fn ready_handler(
        &self,
    ) -> Result<PlayerAction, tokio::io::Error> {
        Ok(PlayerAction::Ready)
    }

    pub async fn play_handler(
        &self,
    ) -> Result<PlayerAction, tokio::io::Error> {
        let mut reader = self.reader.lock().await;

        let mut buf = [0u8; 2];

        reader.read_exact(&mut buf).await?;

        Ok(PlayerAction::Play(Position::new(buf[0], buf[1])))
    }

    pub async fn player_description_handler(
        &self,
    ) -> Result<PlayerAction, tokio::io::Error> {
        let info = HashMap::new();

        Ok(PlayerAction::Metadata(PlayerMetadata{
            info: info,
        }))
    }

    pub async fn unknown_handler(
        &self,
    ) -> Result<PlayerAction, tokio::io::Error> {
        Ok(PlayerAction::Unknown(String::from("")))
    }

    pub async fn error_handler(
        &self,
    ) -> Result<PlayerAction, tokio::io::Error> {
        let mut reader = self.reader.lock().await;

        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf).await?;
        let payload_size = u32::from_be_bytes(buf) as usize;

        let mut buf = vec![0u8; payload_size];
        reader.read_exact(&mut buf).await?;
        let msg = String::from_utf8(buf).expect("from utf-8 error");

        Ok(PlayerAction::Error(msg))
    }

    pub async fn message_handler(
        &self,
    ) -> Result<PlayerAction, tokio::io::Error> {
        let mut reader = self.reader.lock().await;

        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf).await?;
        let payload_size = u32::from_be_bytes(buf) as usize;

        let mut buf = vec![0u8; payload_size];
        reader.read_exact(&mut buf).await?;
        let msg = String::from_utf8(buf).expect("from utf-8 error");

        Ok(PlayerAction::Message(msg))
    }

    pub async fn debug_handler(
        &self,
    ) -> Result<PlayerAction, tokio::io::Error> {
        let mut reader = self.reader.lock().await;

        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf).await?;
        let payload_size = u32::from_be_bytes(buf) as usize;

        let mut buf = vec![0u8; payload_size];
        reader.read_exact(&mut buf).await?;
        let msg = String::from_utf8(buf).expect("from utf-8 error");

        Ok(PlayerAction::Debug(msg))
    }

    pub async fn suggestion_handler(
        &self,
    ) -> Result<PlayerAction, tokio::io::Error> {
        let mut reader = self.reader.lock().await;

        let mut buf = [0u8; 2];

        reader.read_exact(&mut buf).await?;

        Ok(PlayerAction::Suggestion(Position::new(buf[0], buf[1])))
    }
}
