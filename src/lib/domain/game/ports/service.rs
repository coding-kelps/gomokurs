use crate::domain::game::models::GameEnd;
use crate::domain::game::ports::PlayerNotifier;

pub trait GameService<N>
where
    N: PlayerNotifier
{
    fn play(
        &mut self,
    ) -> impl std::future::Future<Output = Result<GameEnd, ()>>;
}
