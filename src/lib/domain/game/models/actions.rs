use crate::domain::game::models::{board::*, game::*};

pub enum PlayerAction {
    Ok,
    Play(Position),
    Description(PlayerInformations),
    Unknown(String),
    Error(String),
    Message(String),
    Debug(String),
    Suggestion(Position),
}
