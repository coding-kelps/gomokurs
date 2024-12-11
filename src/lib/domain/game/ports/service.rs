use crate::domain::game::ports::PlayerClient;

pub trait GameService<C>
where
    C: PlayerClient
{
    fn play(
        &mut self,
    ) -> impl std::future::Future<Output = Result<(), ()>>;
}
