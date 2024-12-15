use crate::domain::board_state_manager::models::{PlayerColor, Position, BoardSize, GameEnd, Error};

pub trait BoardStateManagerService
{
    fn get_size(
        &self,
    ) -> impl std::future::Future<Output = BoardSize>;

    fn play_move(
        &mut self,
        color: PlayerColor,
        position: Position,
    ) -> impl std::future::Future<Output = Result<Option<GameEnd>, Error>>;
}
