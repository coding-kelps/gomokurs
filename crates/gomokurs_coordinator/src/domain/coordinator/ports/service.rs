//! PlayerInterfacesManagerService Port Definition
//!
//! This module defines the PlayerInterfacesManagerService port, responsible for
//! listening to the actions of both the black and white players and forwarding
//! them to the GameManagerService for handling.

use crate::domain::coordinator::models::{GameEnd, Error};
use crate::domain::coordinator::ports::PlayerInterface;
pub use gomokurs_game_engine::domain::game_engine::ports::GameEngineService;

/// A service that coordinates player interactions with the game manager.
///
/// The `PlayerInterfacesManagerService` trait specifies the behavior required
/// to manage interactions between player interfaces and the game manager. It
/// acts as a mediator, coordinating both [`PlayerListener`] instances (one for
/// each player) and invoking the appropriate handlers in the
/// [`GameManagerService`]. Additionally, it can manage timing-related events if
/// necessary.
pub trait CoordinatorService<G, I>
where
    G: GameEngineService,
    I: PlayerInterface,
{
    fn run(
        &mut self,
    ) -> impl std::future::Future<Output = Result<GameEnd, Error>>;
}
