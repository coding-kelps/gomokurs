//! Define the PlayerInterfacesManagerService port.
//! 
//! This module define the PlayerInterfacesManagerService port which role is to
//! listen to both black and white player actions and forward them to the
//! game manager.

use crate::domain::board_state_manager::models::GameEnd;
use crate::domain::game_manager::ports::GameManagerService;
use crate::domain::player_interfaces_manager::ports::PlayerListener;
use crate::domain::game_manager::ports::PlayerNotifier;
use crate::domain::player_interfaces_manager::models::Error;
use std::sync::Arc;

/// Listen to the actions of both players to forward them to the game
/// manager, also handling timers end if any.
/// 
/// This trait define the expected behavior of the player interfaces
/// manager service. The player interfaces manager service is a service
/// meant to coordinates both [`PlayerListener`] to call the appropriate
/// [`GameManagerService`] handler.
pub trait PlayerInterfacesManagerService<I, G>
where
    I: PlayerListener + PlayerNotifier,
    G: GameManagerService,
{
    /// Run a game from a given game_manager, coordinating both white and black
    /// players interface concurrent inputs for the game manager, eventually
    /// returns the game ending.
    /// 
    /// # Arguments
    /// 
    /// * `black_interface` - An Arc to a player interface of what will be
    /// considered the black player during the game.
    /// * `white_interface` - An Arc to a player interface of what will be
    /// considered the white player during the game.
    /// * `game_manager` - A game manager
    fn run(
        &mut self,
        black_interface: Arc<I>,
        white_interface: Arc<I>,
        game_manager: G,
    ) -> impl std::future::Future<Output = Result<GameEnd, Error>>;
}
