use crate::domain::game::models::*;
use crate::domain::game::ports::PlayerNotifier;

pub trait GameService<N>
where
    N: PlayerNotifier
{
    fn register_ok(
        &mut self,
        player: PlayerColor,
    ) -> impl std::future::Future<Output = Result<(), ()>>;

    fn register_move(
        &mut self,
        position: Position,
        player: PlayerColor,
    ) -> impl std::future::Future<Output = Result<(), ()>>;

    fn register_description(
        &mut self,
        description: PlayerInformations,
        player: PlayerColor,
    ) -> impl std::future::Future<Output = Result<(), ()>>;

    fn register_unknown(
        &mut self,
        content: String,
        player: PlayerColor,
    ) -> impl std::future::Future<Output = Result<(), ()>>;

    fn register_error(
        &mut self,
        content: String,
        player: PlayerColor,
    ) -> impl std::future::Future<Output = Result<(), ()>>;

    fn register_message(
        &mut self,
        content: String,
        player: PlayerColor,
    ) -> impl std::future::Future<Output = Result<(), ()>>;

    fn register_debug(
        &mut self,
        content: String,
        player: PlayerColor,
    ) -> impl std::future::Future<Output = Result<(), ()>>;

    fn register_suggestion(
        &mut self,
        position: Position,
        player: PlayerColor,
    ) -> impl std::future::Future<Output = Result<(), ()>>;
}
