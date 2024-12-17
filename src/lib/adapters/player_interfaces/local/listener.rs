use crate::adapters::player_interfaces::local::Local;
use crate::domain::player_interfaces_manager::ports::PlayerListener;
use crate::domain::player_interfaces_manager::models::ListenError;
use crate::domain::game_manager::models::{PlayerColor, PlayerAction};
use tokio::sync::mpsc::Sender;
use anyhow::anyhow;
use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;
use crate::adapters::player_interfaces::local::parsers::*;

impl PlayerListener for Local {
    async fn listen(
        &self,
        player: PlayerColor,
        tx: Sender<(PlayerColor, PlayerAction)>,
    ) -> Result<(), ListenError> {
        loop {
            let line = {
                let mut locked_reader = self.reader.lock().await;
                
                locked_reader.next_line()
                    .await
                    .map_err(|e| anyhow!(e))?
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
