use crate::domain::board_state_manager::models::GameEnd;
use crate::domain::game::ports::GameService;
use crate::domain::game_manager::ports::PlayerListener;
use crate::domain::game::ports::PlayerNotifier;
use crate::domain::game_manager::models::Error;
use std::sync::Arc;

pub trait GameManagerService<C, G>
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
