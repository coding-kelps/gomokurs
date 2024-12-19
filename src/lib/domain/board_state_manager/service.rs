//! This module contains the implementation of the Board State Manager service.

use crate::domain::board_state_manager::ports::BoardStateManagerService;
use crate::domain::board_state_manager::models::{Board, PlayerColor, Position, GameEnd, BoardSize, Error};

/// Represents the Board State Manager service that is responsible for managing
/// the game board state, handling player turns, and determining the game
/// status.
pub struct Service
{
    board: Board,
    turn_player: PlayerColor,
}

impl Service
{
    /// Creates a new instance of the Board State Manager service with the given
    /// board size. The board is initialized, and the first player is set to
    /// `PlayerColor::Black`.
    ///
    /// # Arguments
    ///
    /// * `size` - The dimensions of the board (width and height) that the 
    ///           Board State Manager will handle.
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