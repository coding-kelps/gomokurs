use crate::domain::game::models::{StartRequest, PlayTurnRequest};

pub trait GameService {
    fn start(&self, req: &StartRequest) -> Result<(), ()>;

    fn play_turn(&mut self, req: &PlayTurnRequest) -> Result<(), ()>;
}
