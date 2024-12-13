pub use crate::domain::game::ports::{PlayerClient, GameService};
pub use crate::domain::game_manager::models::Error;
use crate::domain::gomoku::models::GameEnd;
use std::sync::Arc;

pub trait GameManagerService<PC, GS>
where
    PC: PlayerClient,
    GS: GameService,
{
    fn run(
        &mut self,
        black_client: Arc<PC>,
        white_client: Arc<PC>,
        game: GS,
    ) -> impl std::future::Future<Output = Result<GameEnd, Error>>;
}
