use std::collections::HashMap;
use uuid::Uuid;
use crate::domain::game_manager::models::*;
use crate::domain::game_manager::ports::{GameService, PlayerClient};
use tokio::task::JoinHandle;

#[derive(Debug, Clone)]
pub struct Service<C>
where
    C: PlayerClient,
{
    games: HashMap<Uuid, Join<C>>,
}

impl<C> Service<C>
where
    C: PlayerClient,
{
    pub fn new() -> Self {
        Self {
            games: HashMap::new(),
        }
    }
}

impl<C> GameService<C> for Service<C>
where
    C: PlayerClient,
{
    async fn new_game(
            &mut self,
            game: Game<C>,
        ) -> Result<(), ()> {
        self.games.insert(game.uuid, game);

        Ok(())
    }
}
