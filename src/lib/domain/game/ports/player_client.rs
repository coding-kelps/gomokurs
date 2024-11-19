use crate::domain::game::models::{RequestStartError, RequestTurnError, RequestBeginError, RequestBoardError, RequestInfoError, RequestEndError, RequestAboutError};

pub trait PlayerClient {
    fn request_start(
        &self,
    ) -> Result<(), RequestStartError>;

    fn request_turn(
        &self,
    ) -> Result<(), RequestTurnError>;
    
    fn request_begin(
        &self,
    ) -> Result<(), RequestBeginError>;

    fn request_board(
        &self,
    ) -> Result<(), RequestBoardError>;

    fn request_info(
        &self,
    ) -> Result<(), RequestInfoError>;

    fn request_end(
        &self,
    ) -> Result<(), RequestEndError>;

    fn request_about    (
        &self,
    ) -> Result<(), RequestAboutError>;
}
