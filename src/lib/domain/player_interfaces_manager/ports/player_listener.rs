//! Define the PlayerListener Service port.
//! 
//! This module define the PlayerListener port which role is to listen to player
//! actions.

use crate::domain::game_manager::models::{PlayerColor, PlayerAction};
use crate::domain::player_interfaces_manager::models::ListenError;
use tokio::sync::mpsc::Sender;

/// Listen to a player actions.
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
