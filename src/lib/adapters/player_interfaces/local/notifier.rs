//! This module contains the implementation of the PlayerNotifier port for the
//! local player interface.

use crate::adapters::player_interfaces::local::Local;
use crate::domain::game_manager::ports::PlayerNotifier;
use crate::domain::game_manager::models::{Position, NotifyError, RelativeTurn, Information};
use tokio::io::AsyncWriteExt;
use anyhow::anyhow;

impl PlayerNotifier for Local {
    async fn notify_start(
        &self,
        size: u8,
    ) -> Result<(), NotifyError>
    {
        let mut writer = self.writer.lock().await;

        writer
            .write_all(format!("START {}\n", size).as_bytes())
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        writer
            .flush()
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
            .write_all(format!("TURN {}\n", position).as_bytes())
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        writer
            .flush()
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    async fn notify_begin(
        &self,
    ) -> Result<(), NotifyError>
    {
        let mut writer = self.writer.lock().await;

        writer.write_all(b"BEGIN\n")
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        writer.flush()
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

        writer.write_all(b"BOARD\n")
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        for turn in turns {
            writer.write_all(format!("{}\n", turn).as_bytes())
                .await
                .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;
        }

        writer.write_all(b"DONE\n")
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

        writer
            .write_all(format!("INFO {}\n", info).as_bytes())
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        writer
            .flush()
            .await
            .map_err(|e| NotifyError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    async fn notify_end(
        &self,
    ) -> Result<(), NotifyError>
    {
        let mut writer = self.writer.lock().await;

        let _ = writer.write_all(b"END");

        Ok(())
    }

    async fn notify_about(
        &self,
    ) -> Result<(), NotifyError>
    {
        let mut writer = self.writer.lock().await;

        let _ = writer.write_all(b"ABOUT");

        Ok(())
    }

    async fn notify_unknown(
        &self,
        content: &str,
    ) -> Result<(), NotifyError>
    {
        let mut writer = self.writer.lock().await;

        let _ = writer.write_all(format!("UNKNOWN {}", content).as_bytes());

        Ok(())
    }

    async fn notify_error(
        &self,
        content: &str,
    ) -> Result<(), NotifyError>
    {
        let mut writer = self.writer.lock().await;

        let _ = writer.write_all(format!("ERROR {}", content).as_bytes());

        Ok(())
    }
}

