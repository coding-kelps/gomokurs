use crate::domain::game::models::{PlayTurnRequest, PlayTurnError};

pub trait GameService {
    async fn play_turn(&mut self, req: &PlayTurnRequest) -> Result<(), PlayTurnError>;
}
