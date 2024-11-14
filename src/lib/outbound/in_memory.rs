use crate::domain::game::ports::GameStateRepository;
use crate::domain::game::models::{Board, CellStatus, Position, GetBoardError, RegisterTurnError};
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct InMemory {
    board: Board,
}

impl InMemory {
    pub fn new(size: u8) -> Self {
        Self {
            board: Board::new(size),
        }
    }
}

impl GameStateRepository for InMemory {
    fn get_board(&self) -> Result<Board, GetBoardError> {
        Ok(self.board.clone())
    }

    fn register_turn(&mut self, position: Position, status: CellStatus) -> Result<(), RegisterTurnError> {
        match self.board.set_cell(position, status) {
            Err(e) => Err(RegisterTurnError::Unknown(anyhow!(e))),
            Ok(_) => Ok(()),
        }
    }
}
