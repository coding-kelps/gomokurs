//! Define the board state manager service port.
//! 
//! This module defines the board state manager service port which role it to
//! manage the gomoku board state, applying changes to it and checking for
//! players win.

use crate::domain::board_state_manager::models::{PlayerColor, Position, BoardSize, GameEnd, Error};

/// Manage gomoku board state, applying changes to it and checking for players
/// win.
pub trait BoardStateManagerService
{
    /// Return the size of the board as a 2D vector.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use gomokurs::domain::board_state_manager::{BoardStateManager, BoardStateManagerService};
    /// # use gomokurs::domain::board_state_manager::models::{BoardSize};
    /// #
    /// # tokio_test::block_on(async {
    /// let board_size = BoardSize{ x: 20, y: 20 };
    /// let board_state_manager = BoardStateManager::new(board_size);
    /// 
    /// assert_eq!(board_size, board_state_manager.get_size().await);
    /// # });
    fn get_size(
        &self,
    ) -> impl std::future::Future<Output = BoardSize>;

    /// Put a player stone on a cell of the board.
    /// 
    /// # Arguments
    /// 
    /// * `color` - The color of the stone that is put on the board.
    /// * `position` - The position of the cell onto which a stone played.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use gomokurs::domain::board_state_manager::{BoardStateManager, BoardStateManagerService};
    /// # use gomokurs::domain::board_state_manager::models::{PlayerColor, BoardSize, Position, Error};
    /// #
    /// # tokio_test::block_on(async {
    /// let mut board_state_manager = BoardStateManager::new(BoardSize{ x: 20, y: 20 });
    /// 
    /// board_state_manager.play_move(PlayerColor::Black, Position{ x: 1, y: 1}).await?;
    /// # Ok::<(), Error>(())
    /// # });
    /// ```
    fn play_move(
        &mut self,
        color: PlayerColor,
        position: Position,
    ) -> impl std::future::Future<Output = Result<Option<GameEnd>, Error>>;
}
