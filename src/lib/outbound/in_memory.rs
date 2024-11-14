use crate::domain::game::ports::GameStateRepository;
use crate::domain::game::models::{Board, Turn};
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
    fn init_board(&self, size: u8) -> Result<(), GetBoardError> {
        let cells = (0..size)
            .map(|_| vec![CellStatus::Available; size as usize])
            .collect::<Vec<Vec<CellStatus>>>();

        self.board = Board { size, cells };

        Ok(())
    }

    fn get_board(&self) -> Result<Board, GetBoardError> {
        Ok(self.board.clone())
    }

    fn register_turn(&self, turn: &Turn) -> Result<(), RegisterTurnError> {
        match self.board.set_cell(turn.position, turn.status) {
            Err(e) => anyhow!(e).into(),
            Ok(_) => Ok(()),
        }
    }

    fn reset_board(&self) -> Result<(), ResetBoardError> {
        let cells = (0..self.board.size)
            .map(|_| vec![CellStatus::Available; size as usize])
            .collect::<Vec<Vec<CellStatus>>>();

        self.board.cells = cells;

        Ok(())
    }
}
