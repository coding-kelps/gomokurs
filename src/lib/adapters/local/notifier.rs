use crate::adapters::local::local::Local;
use crate::domain::game_manager::models::*;
use crate::domain::game_manager::ports::PlayerNotifier;
use tokio::io::AsyncWriteExt;
use anyhow::anyhow;

impl PlayerNotifier for Local {
    async fn notify_start(
        &mut self,
        size: u8,
    ) -> Result<(), NotifyStartError>
    {
        self.writer
            .write_all(format!("START {}\n", size).as_bytes())
            .await
            .map_err(|e| NotifyStartError::Unknown(anyhow!(e)))?;

        self.writer
            .flush()
            .await
            .map_err(|e| NotifyStartError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    async fn notify_turn(
        &mut self,
        position: Position,
    ) -> Result<(), NotifyTurnError>
    {
        self.writer
            .write_all(format!("TURN {}\n", position).as_bytes())
            .await
            .map_err(|e| NotifyTurnError::Unknown(anyhow!(e)))?;

        self.writer
            .flush()
            .await
            .map_err(|e| NotifyTurnError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    async fn notify_begin(
        &mut self,
    ) -> Result<(), NotifyBeginError>
    {
        self.writer.write_all(b"BEGIN\n")
            .await
            .map_err(|e| NotifyBeginError::Unknown(anyhow!(e)))?;

        self.writer.flush()
            .await
            .map_err(|e| NotifyBeginError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    async fn notify_board(
        &mut self,
        turns: Vec<RelativeTurn>,
    ) -> Result<(), NotifyBoardError>
    {
        self.writer.write_all(b"BOARD\n")
            .await
            .map_err(|e| NotifyBoardError::Unknown(anyhow!(e)))?;

        for turn in turns {
            self.writer.write_all(format!("{}\n", turn).as_bytes())
                .await
                .map_err(|e| NotifyBoardError::Unknown(anyhow!(e)))?;
        }

        self.writer.write_all(b"DONE\n")
            .await
            .map_err(|e| NotifyBoardError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    async fn notify_info(
        &mut self,
        info: Information,
    ) -> Result<(), NotifyInfoError>
    {
        self.writer
            .write_all(format!("INFO {}\n", info).as_bytes())
            .await
            .map_err(|e| NotifyInfoError::Unknown(anyhow!(e)))?;

        self.writer
            .flush()
            .await
            .map_err(|e| NotifyInfoError::Unknown(anyhow!(e)))?;

        Ok(())
    }

    async fn notify_end(
        &mut self,
    ) -> Result<(), NotifyEndError>
    {
        let _ = self.writer.write_all(b"END");

        Ok(())
    }

    async fn notify_about(
        &mut self,
    ) -> Result<(), NotifyAboutError>
    {
        let _ = self.writer.write_all(b"ABOUT");

        Ok(())
    }
}
