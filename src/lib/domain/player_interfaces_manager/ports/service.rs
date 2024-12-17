//!  Define the PlayerInterfacesManagerService port.
//! 
//! This module define the PlayerInterfacesManagerService port which role is to
//! listen to both black and white player actions and call the appropriate
//! handlers of the [`GameManagerService`].

use crate::domain::board_state_manager::models::GameEnd;
use crate::domain::game_manager::ports::GameManagerService;
use crate::domain::player_interfaces_manager::ports::PlayerListener;
use crate::domain::game_manager::ports::PlayerNotifier;
use crate::domain::player_interfaces_manager::models::Error;
use std::sync::Arc;

pub trait PlayerInterfacesManagerService<I, G>
where
    I: PlayerListener + PlayerNotifier,
    G: GameManagerService,
{
    fn run(
        &mut self,
        black_interface: Arc<I>,
        white_interface: Arc<I>,
        game_manager: G,
    ) -> impl std::future::Future<Output = Result<GameEnd, Error>>;
}
