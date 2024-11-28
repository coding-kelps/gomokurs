use std::fmt;
use std::collections::HashMap;
use std::path::PathBuf;
use crate::domain::game_manager::models::board::*;
use crate::domain::game_manager::models::requests::*;
use crate::domain::game_manager::ports::PlayerClient;
use thiserror::Error;
use uuid::Uuid;
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct Game<C>
where
    C: PlayerClient
{
    pub uuid: Uuid,
    black_player: C,
    white_player: C,
    board: Board,
    player_turn: Player,
    _turn_history: Vec<Turn>,
}

impl<C> Game<C>
where
    C: PlayerClient
{
    pub fn new(
        black_player: C,
        white_player: C,
        size: u8,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            black_player: black_player,
            white_player: white_player,
            board: Board::new(size),
            player_turn: Player::Black,
            _turn_history: vec![],
        }
    }

    pub async fn play(
        &mut self,
    ) -> Result<GameEnd, PlayError> {
        let mut last_move = self.black_player.request_begin().await
            .map_err(|e: RequestBeginError| PlayError::Unknown(anyhow!(e)))?;

        self.board.set_cell(last_move, CellStatus::Black)
            .map_err(|e: SetCellError| PlayError::Unknown(anyhow!(e)))?;

        for _ in 1..(self.board.size as u32).pow(2) {
            self.player_turn = self.player_turn.switch();
            let client = match self.player_turn {
                Player::Black => &mut self.black_player,
                Player::White => &mut self.white_player,
            };
            last_move = client.request_turn(last_move).await
                .map_err(|e: RequestTurnError| PlayError::Unknown(anyhow!(e)))?;

            self.board.set_cell(last_move, self.player_turn.into())
                .map_err(|e: SetCellError| PlayError::Unknown(anyhow!(e)))?;

            if self.board.check_win(last_move, self.player_turn.into()) {
                for c in [& mut self.black_player, & mut self.white_player] {
                    c.request_end()
                        .await
                        .map_err(|e: RequestEndError| PlayError::Unknown(anyhow!(e)))?;
                }

                return Ok(GameEnd::Win(self.player_turn))
            }
        }

        Ok(GameEnd::Draw)
    }
}

#[derive(Debug, Error)]
pub enum PlayError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    Black,
    White,
}

impl Player
{
    pub fn switch(&self) -> Player
    {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}

impl Into<CellStatus> for Player
{
    fn into(self) -> CellStatus
    {
        match self {
            Player::Black => CellStatus::Black,
            Player::White => CellStatus::White,
        }
    }
}

impl fmt::Display for Player
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Player::Black =>  write!(f, "black player"),
            Player::White => write!(f, "white player"),
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
    pub player: Player,
}

#[derive(Debug, Clone)]
pub enum GameEnd {
    Win(Player),
    Draw,
}
