use crate::domain::game::models::{board::*, game::*};

#[derive(Debug, Clone)]
pub enum PlayerAction {
    Ok,
    Play(Position),
    Description(PlayerDescription),
    Unknown(String),
    Error(String),
    Message(String),
    Debug(String),
    Suggestion(Position),
}
