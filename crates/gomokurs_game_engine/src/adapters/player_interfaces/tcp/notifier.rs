//! This module contains the implementation of the PlayerNotifier port for the
//! tcp player interface.

use crate::adapters::player_interfaces::tcp::tcp::{Tcp, ActionID};
use crate::domain::game_manager::ports::PlayerNotifier;
use crate::domain::game_manager::models::{Information, NotifyError, Position, RelativeField, RelativeTurn};
use tokio::io::AsyncWriteExt;
use anyhow::anyhow;

impl PlayerNotifier for Tcp {
    async fn notify_start(
        &self,
        size: u8,
    ) -> Result<(), NotifyError>
    {
        let mut writer = self.writer.lock().await;

        writer
            .write_all(&[ActionID::START, size])
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
            .write_all(&[ActionID::TURN, position.x, position.y])
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
            .write_all(&[ActionID::BEGIN])
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
            .write_all(&[ActionID::BOARD])
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        for turn in turns {
            let field = match turn.field {
                RelativeField::OwnStone => 0,
                RelativeField::OpponentStone => 1,
            };

            writer
                .write_all(&[ActionID::BOARD_TURN, turn.position.x, turn.position.y, field])
                .await
                .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;
        }

        writer
            .write_all(&[ActionID::BOARD_END])
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

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

        let data = [&[ActionID::INFO], bytes_info_len, bytes_info].concat();

        writer
            .write_all(&data)
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
            .write_all(&[ActionID::END])
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
            .write_all(&[ActionID::ABOUT])
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
