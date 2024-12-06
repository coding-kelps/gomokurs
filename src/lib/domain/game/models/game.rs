use std::fmt;
use std::collections::HashMap;
use std::hash::Hash;
use std::path::PathBuf;
use crate::domain::game::models::board::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerColor {
    Black,
    White,
}

impl PlayerColor
{
    pub fn other(&self) -> PlayerColor
    {
        match self {
            PlayerColor::Black => PlayerColor::White,
            PlayerColor::White => PlayerColor::Black,
        }
    }

    pub fn switch(&mut self)
    {
        *self = match *self {
            PlayerColor::Black => PlayerColor::White,
            PlayerColor::White => PlayerColor::Black,
        }
    }
}

impl Into<CellStatus> for PlayerColor
{
    fn into(self) -> CellStatus
    {
        match self {
            PlayerColor::Black => CellStatus::Black,
            PlayerColor::White => CellStatus::White,
        }
    }
}

impl fmt::Display for PlayerColor
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            PlayerColor::Black =>  write!(f, "black player"),
            PlayerColor::White => write!(f, "white player"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerInformations {
    pub info: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum Information {
    TimeoutTurn(u64),
    TimeoutMatch(u64),
    MaxMemory(u64),
    TimeLeft(u64),
    GameType(u8),
    Rule(u8),
    Evaluate{
        x: i32,
        y: i32,
    },
    Folder(PathBuf),
}

impl fmt::Display for Information {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Information::TimeoutTurn(t) => write!(f, "timeout_turn {}", t),
            Information::TimeoutMatch(t) => write!(f, "timeout_match {}", t),
            Information::MaxMemory(m) => write!(f, "max_memory {}", m),
            Information::TimeLeft(t) => write!(f, "time_left {}", t),
            Information::GameType(t) => write!(f, "game_type {}", t),
            Information::Rule(r) => write!(f, "rule {}", r),
            Information::Evaluate{x, y} => write!(f, "evaluate {},{}", x, y),
            Information::Folder(p) => {
                let path = p.clone()
                    .into_os_string()
                    .into_string()
                    .expect("failed to convert persistent folder path into str");

                write!(f, "folder {}", path)
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum RelativeField {
    OwnStone,
    OpponentStone,
}

impl fmt::Display for RelativeField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match &self {
            RelativeField::OwnStone => write!(f, "1"),
            RelativeField::OpponentStone => write!(f, "2"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RelativeTurn {
    pub position: Position,
    pub field: RelativeField,
}

impl fmt::Display for RelativeTurn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "{},{}", self.position,self.field)
    }
}

#[derive(Debug, Clone)]
pub struct Turn {
    pub position: Position,
    pub player: PlayerColor,
}

#[derive(Debug, Clone, Copy)]
pub enum GameEnd {
    Win(PlayerColor),
    Draw,
}
