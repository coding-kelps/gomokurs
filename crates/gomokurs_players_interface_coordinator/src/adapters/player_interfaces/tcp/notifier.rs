//! This module contains the implementation of the PlayerNotifier port for the
//! tcp player interface.

use crate::adapters::player_interfaces::tcp::tcp::{Tcp, ActionID};
use gomokurs_game_engine::domain::game_manager::ports::PlayerNotifier;
use gomokurs_game_engine::domain::game_manager::models::{Information, NotifyError, Position, RelativeField, RelativeTurn, RelativeGameEnd};
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
