use crate::domain::gomoku::models::{PlayerColor, Position, GameEnd, Error};

pub trait GomokuService
{
    fn play_move(
        &mut self,
        color: PlayerColor,
        position: Position,
    ) -> impl std::future::Future<Output = Result<Option<GameEnd>, Error>>;
}
