//! Defines the PlayerNotifier port used for communication with a player.

use crate::domain::board_state_manager::models::Position;
use crate::domain::game_manager::models::{RelativeTurn, Information, NotifyError};

/// A trait representing notifications sent to a player from the game manager.
/// 
/// This trait defines the interface for asynchronously notifying players 
/// about various game events and requesting player-specific information.
/// Implementors of this trait are responsible for handling the asynchronous
/// interactions with a player.
pub trait PlayerNotifier: Send + Sync + 'static {
    /// Notifies the player of the initial gomoku board configuration.
    /// 
    /// # Arguments
    /// 
    /// * `size` - The size of the gomoku board, both width and height.
    fn notify_start(
        &self,
        size: u8,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    /// Notifies the player about the opponent's turn.
    /// 
    /// # Arguments
    /// 
    /// * `position` - The position where the opponent placed their piece.
    fn notify_turn(
        &self,
        position: Position,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;
    
    /// Notifies the player that it is their turn to make the first move.
    fn notify_begin(
        &self,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    /// Notifies the player of the initial gomoku board state, including
    /// any moves that have already been played.
    /// 
    /// # Arguments
    /// 
    /// * `turns` - A collection of moves (`RelativeTurn`) that have been played.
    fn notify_board(
        &self,
        turns: Vec<RelativeTurn>,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    /// Sends general game-related information to the player.
    /// 
    /// # Arguments
    /// 
    /// * `info` - The game information to send.
    fn notify_info(
        &self,
        info: Information,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    /// Notifies the player that the game has finished.
    fn notify_end(
        &self,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    /// Requests a description of the player from the player.
    fn notify_about(
        &self,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    /// Notifies the player that their last action was unrecognized.
    /// 
    /// # Arguments
    /// 
    /// * `content` - A message explaining the error or unrecognized action.
    fn notify_unknown(
        &self,
        content: &str,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    /// Notifies the player that their last action or its arguments were unexpected.
    /// 
    /// # Arguments
    /// 
    /// * `content` - A message explaining the unexpected action or arguments.
    fn notify_error(
        &self,
        content: &str,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;
}
