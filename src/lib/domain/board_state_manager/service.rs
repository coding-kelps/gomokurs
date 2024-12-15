use crate::domain::board_state_manager::ports::BoardStateManagerService;
use crate::domain::board_state_manager::models::{Board, PlayerColor, Position, GameEnd, Error};

use super::models::BoardSize;

pub struct Service
{
    board: Board,
    turn_player: PlayerColor,
}

impl Service
{
    pub fn new(
        size: BoardSize,
    ) -> Self
    {
        Self {
            board: Board::new(size),
            turn_player: PlayerColor::Black,
        }
    }
}

impl BoardStateManagerService for Service
{
    async fn get_size(
            &self,
        ) -> BoardSize {
        self.board.size
    }

    async fn play_move(
        &mut self,
        color: PlayerColor,
        position: Position,
    ) -> Result<Option<GameEnd>, Error>
    {            
        if color != self.turn_player {
            Err(Error::NotPlayerTurn(color))
        } else {
            self.board.set_cell(position, color.into())?;

            if self.board.check_win(position, color.into()) {
                Ok(Some(GameEnd::Win(color)))
            } else {
                self.turn_player.switch();

                Ok(None)
            }
        }
    }
}