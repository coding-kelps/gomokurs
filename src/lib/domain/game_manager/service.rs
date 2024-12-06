use crate::domain::game_manager::ports::GameManagerService;

#[derive()]
pub struct Service
{
}

impl Service
{
    pub fn new() -> Self {
        Self {
        }
    }
}

impl GameManagerService for Service {
    async fn new_game(
            &mut self,
        ) -> Result<(), ()> {
            Ok(())
    }
}
