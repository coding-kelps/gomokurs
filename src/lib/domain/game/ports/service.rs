use crate::domain::gomoku::models::{Position, GameEnd};
use crate::domain::game::models::{Error, PlayerColor, PlayerDescription};

pub trait GameService
{
    fn init_game(
        &self,
    ) -> impl std::future::Future<Output = Result<(), Error>>;

    fn handle_ok(
        &mut self,
        color: PlayerColor,
    ) -> impl std::future::Future<Output = Result<(), Error>>;

    fn handle_play(
        &mut self,
        color: PlayerColor,
        position: Position,
    ) -> impl std::future::Future<Output = Result<Option<GameEnd>, Error>>;

    fn handle_description(
        &mut self,
        color: PlayerColor,
        description: PlayerDescription,
    ) -> impl std::future::Future<Output = Result<(), Error>>;

    fn handle_unknown(
        &self,
        color: PlayerColor,
        content: String,
    ) -> impl std::future::Future<Output = Result<(), Error>>;

    fn handle_error(
        &self,
        color: PlayerColor,
        content: String,
    ) -> impl std::future::Future<Output = Result<(), Error>>;

    fn handle_message(
        &self,
        color: PlayerColor,
        content: String,
    ) -> impl std::future::Future<Output = Result<(), Error>>;

    fn handle_debug(
        &self,
        color: PlayerColor,
        content: String,
    ) -> impl std::future::Future<Output = Result<(), Error>>;

    fn handle_suggestion(
        &self,
        color: PlayerColor,
        position: Position,
    ) -> impl std::future::Future<Output = Result<(), Error>>;
}
