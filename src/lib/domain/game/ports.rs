use crate::domain::game::models::{Turn, InitBoardError, GetBoardError, RegisterTurnError, ResetBoardError};

pub trait GameStateRepository {
    fn init_board(
        &self,
        size: u8,
    ) -> Result<(), InitBoardError>;

    fn get_board(
        &self,
    ) -> Result<(), GetBoardError>;

    fn register_turn(
        &self,
        turn: &Turn,
    ) -> Result<(), RegisterTurnError>;

    fn reset_board(
        &self,
    ) -> Result<(), ResetBoardError>;
}
