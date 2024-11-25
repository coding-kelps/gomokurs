use crate::domain::game::models::{GameEnd, PlayError, PlayRequest};

pub trait GameService {
    async fn play(&mut self, _req: &PlayRequest) -> Result<GameEnd, PlayError>;
}
