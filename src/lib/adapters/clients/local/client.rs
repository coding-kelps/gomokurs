use crate::adapters::clients::local::local::Local;
use crate::adapters::clients::local::parsers::*;
use crate::domain::game::models::*;
use crate::domain::game::ports::PlayerClient;
use lazy_static::lazy_static;
use regex::Regex;
use anyhow::anyhow;
use thiserror::Error;
use tokio::sync::mpsc::Sender;
use tokio::io::AsyncWriteExt;

impl PlayerClient for Local {
    async fn listen(
        &self,
        tx: Sender<(PlayerColor, PlayerAction)>,
        player: PlayerColor,
    ) -> Result<(), ()> {
        loop {
            let line = {
                let mut locked_reader = self.reader.lock().await;
                
                locked_reader.next_line()
                    .await
                    .map_err(|_| ())?
                    .expect("self.reader.next_line() results is None")
            };

            match PlayerAction::try_from(line) {
                Ok(action) => {
                    tx.send((player, action)).await.expect("failed to send to channel");
                },
                Err(e) => {
                    println!("error at convertion {:?}", e);
                }
            };
        }
    }

    async fn notify_start(
        &self,
        size: u8,
    ) -> Result<(), NotifyStartError>
    {
        let mut writer = self.writer.lock().await;

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
        let mut writer = self.writer.lock().await;

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
        let mut writer = self.writer.lock().await;

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
        let mut writer = self.writer.lock().await;

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
        let mut writer = self.writer.lock().await;

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
        let mut writer = self.writer.lock().await;

        let _ = writer.write_all(b"END");

        Ok(())
    }

    async fn notify_about(
        &self,
    ) -> Result<(), NotifyAboutError>
    {
        let mut writer = self.writer.lock().await;

        let _ = writer.write_all(b"ABOUT");

        Ok(())
    }

    async fn notify_unknown(
        &self,
        content: &str,
    ) -> Result<(), NotifyUnknownError>
    {
        let mut writer = self.writer.lock().await;

        let _ = writer.write_all(format!("UNKNOWN {}", content).as_bytes());

        Ok(())
    }

    async fn notify_error(
        &self,
        content: &str,
    ) -> Result<(), NotifyErrorError>
    {
        let mut writer = self.writer.lock().await;

        let _ = writer.write_all(format!("ERROR {}", content).as_bytes());

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum InputConvertionError {
    #[error("unknown command")]
    UnknownCommand,
    #[error("parsing error: `{0}`")]
    ParsingError(#[from] anyhow::Error),
}

impl TryFrom<String> for PlayerAction {
    type Error = InputConvertionError;

    fn try_from(value: String) -> Result<Self, <PlayerAction as TryFrom<String>>::Error> {
        lazy_static! {
            static ref RE_OK: Regex = Regex::new(r"^OK$")
                .expect("failed to initiate ok command regex!");
            static ref RE_PLAY: Regex = Regex::new(r"^\d+,\d+$")
                .expect("failed to initiate play command regex!");
            static ref RE_DESC: Regex = Regex::new(r#"[\w\-\_]+="[^"]*""#)
                .expect("failed to initiate player description command regex!");
            static ref RE_UNK: Regex = Regex::new(r"UNKNOWN .*")
                .expect("failed to initiate unknown command regex!");
            static ref RE_ERR: Regex = Regex::new(r"ERROR .*")
                .expect("failed to initiate error command regex!");
            static ref RE_MSG: Regex = Regex::new(r"MESSAGE .*")
                .expect("failed to initiate message command regex!");
            static ref RE_DBG: Regex = Regex::new(r"DEBUG .*")
                .expect("failed to initiate debug command regex!");
            static ref RE_SGT: Regex = Regex::new(r"SUGGEST .*")
                .expect("failed to initiate suggestion command regex!");
        }

        if RE_OK.is_match(&value) {
            Ok(PlayerAction::Ok)
        } else if RE_PLAY.is_match(&value) {
            let position = parse_position(&value)
                .map_err(|e| anyhow!(e))?;

            Ok(PlayerAction::Play(position))
        } else if RE_DESC.is_match(&value) {
            let infos = parse_player_informations(&value);

            Ok(PlayerAction::Description(infos))
        } else if RE_UNK.is_match(&value) {
            let content = parse_content(&value)
                .map_err(|e| anyhow!(e))?;

            Ok(PlayerAction::Unknown(content))
        } else if RE_ERR.is_match(&value) {
            let content = parse_content(&value)
                .map_err(|e| anyhow!(e))?;

            Ok(PlayerAction::Error(content))
        } else if RE_MSG.is_match(&value) {
            let content = parse_content(&value)
                .map_err(|e| anyhow!(e))?;

            Ok(PlayerAction::Message(content))
        } else if RE_DBG.is_match(&value) {
            let content = parse_content(&value)
                .map_err(|e| anyhow!(e))?;

            Ok(PlayerAction::Debug(content))
        } else {
            Err(InputConvertionError::UnknownCommand)
        }
    }
}

