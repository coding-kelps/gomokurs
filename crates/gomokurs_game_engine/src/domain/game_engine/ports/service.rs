use crate::domain::game_engine::models::*;

pub trait GameEngineService
{
    fn get_board_size(
        &self,
    ) -> impl std::future::Future<Output = BoardSize>;

    fn run_timers(
        &self,
    ) -> impl std::future::Future<Output = Result<GameEnd, Error>>;

    fn register_move(
        &mut self,
        color: PlayerColor,
        position: Position,
    ) -> impl std::future::Future<Output = Result<Option<GameEnd>, Error>>;
}
