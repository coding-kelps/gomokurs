use crate::domain::game::models::{Information, PlayerInformations, Position, RelativeTurn, RequestAboutError, RequestBeginError, RequestBoardError, RequestEndError, RequestInfoError, RequestStartError, RequestTurnError};

pub trait PlayerClient {
    async fn request_start(
        &mut self,
        size: u8,
    ) -> Result<(), RequestStartError>;

    async fn request_turn(
        &mut self,
        position: Position,
    ) -> Result<Position, RequestTurnError>;
    
    async fn request_begin(
        &mut self,
    ) -> Result<Position, RequestBeginError>;

    async fn request_board(
        &mut self,
        turns: Vec<RelativeTurn>,
    ) -> Result<Position, RequestBoardError>;

    async fn request_info(
        &mut self,
        info: Information,
    ) -> Result<(), RequestInfoError>;

    async fn request_end(
        &mut self,
    ) -> Result<(), RequestEndError>;

    async fn request_about(
        &mut self,
    ) -> Result<PlayerInformations, RequestAboutError>;
}
