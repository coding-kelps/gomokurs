//! This module contains the implementation of the PlayerListener port for the
//! tcp player interface.

use crate::adapters::player_interfaces::tcp::Tcp;
use crate::domain::player_interfaces_manager::ports::PlayerListener;
use crate::domain::player_interfaces_manager::models::ListenError;
use crate::domain::game_manager::models::{PlayerColor, PlayerAction};
use tokio::sync::mpsc::Sender;

impl PlayerListener for Tcp {
    #[allow(unused_variables)]
    async fn listen(
        &self,
        player: PlayerColor,
        tx: Sender<(PlayerColor, PlayerAction)>,
    ) -> Result<(), ListenError> {
        Ok(())
    }
}
