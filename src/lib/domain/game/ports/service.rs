use crate::domain::game::models::*;
use crate::domain::game::ports::PlayerNotifier;

pub trait GameService<N>
where
    N: PlayerNotifier
{
    fn handle_player_action(
        &mut self,
        action: PlayerAction,
        player: PlayerColor,
    ) -> impl std::future::Future<Output = Result<Option<GameEnd>, PlayError>>;
}
