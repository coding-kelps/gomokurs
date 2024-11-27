use crate::domain::game::models::*;

pub trait PlayerClient {
    fn request_start(
        &mut self,
        size: u8,
    ) -> impl std::future::Future<Output = Result<(), RequestStartError>> + Send;

    fn request_turn(
        &mut self,
        position: Position,
    ) -> impl std::future::Future<Output = Result<Position, RequestTurnError>> + Send;
    
    fn request_begin(
        &mut self,
    ) -> impl std::future::Future<Output = Result<Position, RequestBeginError>>;

    fn request_board(
        &mut self,
        turns: Vec<RelativeTurn>,
    ) -> impl std::future::Future<Output = Result<Position, RequestBoardError>>;

    fn request_info(
        &mut self,
        info: Information,
    ) -> impl std::future::Future<Output = Result<(), RequestInfoError>>;

    fn request_end(
        &mut self,
    ) -> impl std::future::Future<Output = Result<(), RequestEndError>>;

    fn request_about(
        &mut self,
    ) -> impl std::future::Future<Output = Result<PlayerInformations, RequestAboutError>>;
}
