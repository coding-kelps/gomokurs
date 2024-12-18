//! An implementation of the board state manager service.

use crate::domain::board_state_manager::ports::BoardStateManagerService;
use crate::domain::board_state_manager::models::{Board, PlayerColor, Position, GameEnd, BoardSize, Error};

/// An implementation of the board state manager service.
pub struct Service
{
    board: Board,
    turn_player: PlayerColor,
}

impl Service
{
    /// Initialize a new board state manager service
    /// from a given board size.
    /// 
    /// # Arguments
    /// 
    /// * `size` - The size of the board (in width and height) that will be
    /// created and managed by the board state manager.
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

            if self.board.check_win(position).await {
                Ok(Some(GameEnd::Win(color)))
            } else {
                self.turn_player.switch();

                Ok(None)
            }
        }
    }
}