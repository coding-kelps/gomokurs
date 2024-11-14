use crate::domain::game::ports::GameStateRepository;
use crate::domain::game::models::{Board, CellStatus, Position, InitBoardError, GetBoardError, RegisterTurnError};
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct InMemory {
    board: Board,
}

impl InMemory {
    pub fn new() -> Self {
        Self {
            board: Board {
                size: 0,
                cells: vec![],
            }
        }
    }
}

impl GameStateRepository for InMemory {
    fn init_board(&self, size: u8) -> Result<(), InitBoardError> {
        let cells = (0..size)
            .map(|_| vec![CellStatus::Available; size as usize])
            .collect::<Vec<Vec<CellStatus>>>();

        self.board = Board { size, cells };

        Ok(())
    }

    fn get_board(&self) -> Result<Board, GetBoardError> {
        Ok(self.board.clone())
    }

    fn register_turn(&mut self, position: &Position, status: &CellStatus) -> Result<(), RegisterTurnError> {
        match self.board.set_cell(position, status) {
            Err(e) => anyhow!(e).into(),
            Ok(_) => Ok(()),
        }
    }
}
