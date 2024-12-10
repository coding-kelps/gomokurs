use crate::adapters::local::local::Local;
use crate::adapters::local::parsers::*;
use crate::domain::game::models::*;
use lazy_static::lazy_static;
use regex::Regex;
use anyhow::anyhow;

impl Local {
    async fn listen_action(
        &self,
    ) -> Result<PlayerAction, ListenActionError> {
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

        let mut reader = self.reader.lock()
            .expect("failed to get reader from mutex");

        let line = reader.next_line()
            .await
            .map_err(|e| ListenActionError::Unknown(anyhow!(e)))?
            .expect("self.reader.next_line() results is None");

        if RE_OK.is_match(&line) {
            Ok(PlayerAction::Ok)
        } else if RE_PLAY.is_match(&line) {
            let position = parse_position(&line)
                .map_err(|e| ListenActionError::Unknown(anyhow!(e)))?;

            Ok(PlayerAction::Play(position))
        } else if RE_DESC.is_match(&line) {
            let infos = parse_player_informations(&line);

            Ok(PlayerAction::Description(infos))
        } else if RE_UNK.is_match(&line) {
            let content = parse_content(&line)
                .map_err(|e| ListenActionError::Unknown(anyhow!(e)))?;

            Ok(PlayerAction::Unknown(content))
        } else if RE_ERR.is_match(&line) {
            let content = parse_content(&line)
                .map_err(|e| ListenActionError::Unknown(anyhow!(e)))?;

            Ok(PlayerAction::Error(content))
        } else if RE_MSG.is_match(&line) {
            let content = parse_content(&line)
                .map_err(|e| ListenActionError::Unknown(anyhow!(e)))?;

            Ok(PlayerAction::Message(content))
        } else if RE_DBG.is_match(&line) {
            let content = parse_content(&line)
                .map_err(|e| ListenActionError::Unknown(anyhow!(e)))?;

            Ok(PlayerAction::Debug(content))
        } else {
            Err(ListenActionError::UnknownCommand)
        }
    }
}
