use crate::adapters::local::local::Local;
use crate::domain::game::models::*;
use crate::domain::game::ports::PlayerNotifier;
use tokio::io::AsyncWriteExt;
use anyhow::anyhow;

impl PlayerNotifier for Local {
    async fn notify_start(
        &self,
        size: u8,
    ) -> Result<(), NotifyStartError>
    {
        let mut writer = self.writer.lock().unwrap();

        writer
            .write_all(format!("START {}\n", size).as_bytes())
            .await
            .map_err(|e| NotifyStartError::Unknown(anyhow!(e)))?;

        writer
            .flush()
            .await
            .map_err(|e| NotifyStartError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    async fn notify_turn(
        &self,
        position: Position,
    ) -> Result<(), NotifyTurnError>
    {
        let mut writer = self.writer.lock().unwrap();

        writer
            .write_all(format!("TURN {}\n", position).as_bytes())
            .await
            .map_err(|e| NotifyTurnError::Unknown(anyhow!(e)))?;

        writer
            .flush()
            .await
            .map_err(|e| NotifyTurnError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    async fn notify_begin(
        &self,
    ) -> Result<(), NotifyBeginError>
    {
        let mut writer = self.writer.lock().unwrap();

        writer.write_all(b"BEGIN\n")
            .await
            .map_err(|e| NotifyBeginError::Unknown(anyhow!(e)))?;

        writer.flush()
            .await
            .map_err(|e| NotifyBeginError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    async fn notify_board(
        &self,
        turns: Vec<RelativeTurn>,
    ) -> Result<(), NotifyBoardError>
    {
        let mut writer = self.writer.lock().unwrap();

        writer.write_all(b"BOARD\n")
            .await
            .map_err(|e| NotifyBoardError::Unknown(anyhow!(e)))?;

        for turn in turns {
            writer.write_all(format!("{}\n", turn).as_bytes())
                .await
                .map_err(|e| NotifyBoardError::Unknown(anyhow!(e)))?;
        }

        writer.write_all(b"DONE\n")
            .await
            .map_err(|e| NotifyBoardError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    async fn notify_info(
        &self,
        info: Information,
    ) -> Result<(), NotifyInfoError>
    {
        let mut writer = self.writer.lock().unwrap();

        writer
            .write_all(format!("INFO {}\n", info).as_bytes())
            .await
            .map_err(|e| NotifyInfoError::Unknown(anyhow!(e)))?;

        writer
            .flush()
            .await
            .map_err(|e| NotifyInfoError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    async fn notify_end(
        &self,
    ) -> Result<(), NotifyEndError>
    {
        let mut writer = self.writer.lock().unwrap();

        let _ = writer.write_all(b"END");

        Ok(())
    }

    async fn notify_about(
        &self,
    ) -> Result<(), NotifyAboutError>
    {
        let mut writer = self.writer.lock().unwrap();

        let _ = writer.write_all(b"ABOUT");

        Ok(())
    }

    async fn notify_unknown(
        &self,
        content: &str,
    ) -> Result<(), NotifyUnknownError>
    {
        let mut writer = self.writer.lock().unwrap();

        let _ = writer.write_all(format!("UNKNOWN {}", content).as_bytes());

        Ok(())
    }

    async fn notify_error(
        &self,
        content: &str,
    ) -> Result<(), NotifyErrorError>
    {
        let mut writer = self.writer.lock().unwrap();

        let _ = writer.write_all(format!("ERROR {}", content).as_bytes());

        Ok(())
    }
}
