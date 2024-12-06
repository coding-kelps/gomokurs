use crate::domain::game_manager::models::{board::*, game::*};

pub enum PlayerCommands {
    Ok,
    Play(Position),
    Description(PlayerInformations),
    Unknown(String),
    Error(String),
    Message(String),
    Debug(String),
    Suggestion(Position),
}

