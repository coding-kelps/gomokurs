use crate::domain::board_state_manager::models::GameEnd;
use crate::domain::game::ports::GameService;
use crate::domain::players_interface::ports::PlayerListener;
use crate::domain::game::ports::PlayerNotifier;
use crate::domain::players_interface::models::Error;
use std::sync::Arc;

pub trait PlayersInterfaceService<C, G>
where
    C: PlayerListener + PlayerNotifier,
    G: GameService,
{
    fn run(
        &mut self,
        black_client: Arc<C>,
        white_client: Arc<C>,
        game: G,
    ) -> impl std::future::Future<Output = Result<GameEnd, Error>>;
}
