use crate::domain::game_manager::models::Game;
use crate::domain::game_manager::ports::PlayerNotifier;

pub trait GameService<N>
where
    N: PlayerNotifier
{
    fn new_game(
        &mut self,
        game: Game<N>,
    ) -> impl std::future::Future<Output = Result<(), ()>>;
}
