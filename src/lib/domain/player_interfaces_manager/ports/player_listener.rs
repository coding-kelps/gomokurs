//! Define the player listener service port.
//! 
//! This module defines the player listener service port which role is to listen to player
//! actions.

use crate::domain::game_manager::models::{PlayerColor, PlayerAction};
use crate::domain::player_interfaces_manager::models::ListenError;
use tokio::sync::mpsc::Sender;

/// Listen to the actions of a player.
/// 
/// This trait define the expected behavior of a player listener.
/// It is meant to listen to a player action and push them to a
/// given channel.
pub trait PlayerListener: Send + Sync + 'static {
    /// listen to player action and push them in a given channel.
    /// 
    /// # Arguments
    /// 
    /// * `color` - The color of the player that is listened to.
    /// * `tx` - A sender to player actions' channel.
    fn listen(
        &self,
        color: PlayerColor,
        tx: Sender<(PlayerColor, PlayerAction)>,
    ) -> impl std::future::Future<Output = Result<(), ListenError>> + Send;
}
