use crate::domain::board_state_manager::models::GameEnd;
use crate::domain::game_manager::ports::GameManagerService;
use crate::domain::player_interfaces_manager::ports::PlayerListener;
use crate::domain::game_manager::ports::PlayerNotifier;
use crate::domain::player_interfaces_manager::models::Error;
use std::sync::Arc;

pub trait PlayerInterfacesManagerService<C, G>
where
    C: PlayerListener + PlayerNotifier,
    G: GameManagerService,
{
    fn run(
        &mut self,
        black_client: Arc<C>,
        white_client: Arc<C>,
        game_manager: G,
    ) -> impl std::future::Future<Output = Result<GameEnd, Error>>;
}
