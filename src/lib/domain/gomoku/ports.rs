use crate::domain::gomoku::models::{PlayerColor, Position, BoardSize, GameEnd, Error};

pub trait GomokuService
{
    fn get_board_size(
        &self,
    ) -> impl std::future::Future<Output = BoardSize>;

    fn play_move(
        &mut self,
        color: PlayerColor,
        position: Position,
    ) -> impl std::future::Future<Output = Result<Option<GameEnd>, Error>>;
}
