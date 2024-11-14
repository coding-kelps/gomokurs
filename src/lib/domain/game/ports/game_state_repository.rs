use crate::domain::game::models::{CellStatus, GetBoardError, InitBoardError, Position, Board, RegisterTurnError};

pub trait GameStateRepository {
    fn init_board(
        &self,
        size: u8,
    ) -> Result<(), InitBoardError>;

    fn get_board(
        &self,
    ) -> Result<Board, GetBoardError>;

    fn register_turn(
        &mut self,
        position: &Position,
        status: &CellStatus,
    ) -> Result<(), RegisterTurnError>;
}
