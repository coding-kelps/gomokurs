use crate::domain::game::models::{CellStatus, GetBoardError, Position, Board, RegisterTurnError};

pub trait GameStateRepository {
    fn get_board(
        &self,
    ) -> Result<Board, GetBoardError>;

    fn register_turn(
        &mut self,
        position: Position,
        status: CellStatus,
    ) -> Result<(), RegisterTurnError>;
}
