//! Player Listener Service Port
//!
//! This module defines the service port for the Player Listener, responsible
//! for listening to player actions and forwarding them to a specified channel.

use gomokurs_game_engine::domain::game_manager::models::{PlayerAction, PlayerColor};
pub use gomokurs_game_engine::domain::game_manager::ports::PlayerNotifier;
use crate::domain::player_interfaces_manager::models::ListenError;
use tokio::sync::mpsc::Sender;

/// A trait for listening to player actions.
///
/// The `PlayerListener` trait outlines the expected behavior of a player
/// listener. It listens for a player's actions and sends them to a specified
/// channel. Implementations of this trait enable communication between the game
/// manager and player interfaces.
///
/// # Responsibilities
/// * Listen to player actions in real-time.
/// * Forward the player's actions along with their associated color to the
/// provided channel.
pub trait PlayerListener: Send + Sync + 'static {
    /// Listens for player actions and sends them to the specified channel.
    ///
    /// # Arguments
    ///
    /// * `color` - The `PlayerColor` representing the player being listened to.
    /// * `tx` - A `Sender` channel for forwarding `(PlayerColor, PlayerAction)`
    /// tuples.
    fn listen(
        &self,
        color: PlayerColor,
        tx: Sender<(PlayerColor, PlayerAction)>,
    ) -> impl std::future::Future<Output = Result<(), ListenError>> + Send;
}
