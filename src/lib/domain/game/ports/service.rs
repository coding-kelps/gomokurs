use crate::domain::game::ports::PlayerClient;
use crate::domain::game::models::{GameEnd, Error};

pub trait GameService<C>
where
    C: PlayerClient
{
    fn play(
        &mut self,
    ) -> impl std::future::Future<Output = Result<GameEnd, Error>>;
}
