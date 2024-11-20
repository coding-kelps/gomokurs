use crate::domain::game::models::{Position, RequestStartError, RequestTurnError, RequestBeginError, RequestBoardError, RequestInfoError, RequestEndError, RequestAboutError};

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
        positions: Vec<Position>,
    ) -> Result<(), RequestBoardError>;

    async fn request_info(
        &mut self,
    ) -> Result<(), RequestInfoError>;

    async fn request_end(
        &mut self,
    ) -> Result<(), RequestEndError>;

    async fn request_about(
        &mut self,
    ) -> Result<(), RequestAboutError>;
}
