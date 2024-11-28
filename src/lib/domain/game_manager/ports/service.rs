use crate::domain::game_manager::models::Game;
use crate::domain::game_manager::ports::PlayerClient;

pub trait GameService<C>
where
    C: PlayerClient
{
    fn new_game(
        &mut self,
        game: Game<C>,
    ) -> impl std::future::Future<Output = Result<(), ()>>;
}
