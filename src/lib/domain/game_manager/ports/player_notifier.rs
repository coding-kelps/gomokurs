//! Define the PlayerNotifier port.

use crate::domain::board_state_manager::models::Position;
use crate::domain::game_manager::models::{RelativeTurn, Information, NotifyError};

/// Notify a player from a manager action.
pub trait PlayerNotifier: Send + Sync + 'static {
    /// Notify the gomoku board initial configuration to the player.
    /// 
    /// # Arguments
    /// 
    /// * `size` - The gomoku board size in both width and height.
    fn notify_start(
        &self,
        size: u8,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    /// Notify the opponent player's turn to the player.
    /// 
    /// # Arguments
    /// 
    /// * `position` - The position at which the player opponent played.
    fn notify_turn(
        &self,
        position: Position,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;
    
    /// Notify to the player that the manager is waiting for it to make the
    /// first move.
    fn notify_begin(
        &self,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    /// Notify the gomoku board initial configuration in which some moves
    /// already have been played.
    /// 
    /// # Arguments
    /// 
    /// * `turns` - The collection of turns that have been played.
    fn notify_board(
        &self,
        turns: Vec<RelativeTurn>,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    /// Notify a game information to the player.
    fn notify_info(
        &self,
        info: Information,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    /// Notify the game is fninshed to the player.
    fn notify_end(
        &self,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    /// Notify the manager wants the player description.
    fn notify_about(
        &self,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    /// Notify to the player that the manager didn't recognize its last action.
    /// 
    /// # Arguments
    /// 
    /// * `content` - The manager's error message content.
    fn notify_unknown(
        &self,
        content: &str,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;

    /// Notify to the player that the manager didn't expect its last action or
    /// its last action arguments.
    /// 
    /// # Arguments
    /// 
    /// * `content` - The manager's error message content.
    fn notify_error(
        &self,
        content: &str,
    ) -> impl std::future::Future<Output = Result<(), NotifyError>>;
}
