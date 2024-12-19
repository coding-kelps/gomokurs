//! PlayerInterfacesManagerService Port Definition
//!
//! This module defines the PlayerInterfacesManagerService port, responsible for
//! listening to the actions of both the black and white players and forwarding
//! them to the GameManagerService for handling.

use crate::domain::board_state_manager::models::GameEnd;
use crate::domain::game_manager::ports::GameManagerService;
use crate::domain::player_interfaces_manager::ports::PlayerListener;
use crate::domain::game_manager::ports::PlayerNotifier;
use crate::domain::player_interfaces_manager::models::Error;
use std::sync::Arc;

/// A service that coordinates player interactions with the game manager.
///
/// The `PlayerInterfacesManagerService` trait specifies the behavior required
/// to manage interactions between player interfaces and the game manager. It
/// acts as a mediator, coordinating both [`PlayerListener`] instances (one for
/// each player) and invoking the appropriate handlers in the
/// [`GameManagerService`]. Additionally, it can manage timing-related events if
/// necessary.
pub trait PlayerInterfacesManagerService<I, G>
where
    I: PlayerListener + PlayerNotifier,
    G: GameManagerService,
{
    /// Runs a game session by managing concurrent inputs from both players
    /// and forwarding them to the game manager. Returns the game's outcome
    /// upon completion.
    ///
    /// # Arguments
    ///
    /// * `black_interface` - A reference-counted pointer to the interface
    ///   representing the black player.
    /// * `white_interface` - A reference-counted pointer to the interface
    ///   representing the white player.
    /// * `game_manager` - The game manager responsible for handling the game
    /// logic.
    fn run(
        &mut self,
        black_interface: Arc<I>,
        white_interface: Arc<I>,
        game_manager: G,
    ) -> impl std::future::Future<Output = Result<GameEnd, Error>>;
}
