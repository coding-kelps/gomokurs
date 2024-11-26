use crate::domain::game::models::{GameEnd, PlayError, PlayRequest};

pub trait GameService {
    fn play(
        &mut self, _req: &PlayRequest
    ) -> impl std::future::Future<Output = Result<GameEnd, PlayError>>;
}
