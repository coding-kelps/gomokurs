use crate::domain::game_manager::ports::GameManagerService;
use gomokurs_game_engine::domain::{player_interfaces_manager::PlayerInterfacesManager, player_interfaces_manager::PlayerInterfacesManagerService, game_manager::GameManager, board_state_manager::BoardStateManager};
use gomokurs_game_engine::domain::{board_state_manager::models::{GameEnd, BoardSize}, player_interfaces_manager::models::Error as PlayerInterfaceManagerError};
use gomokurs_game_engine::adapters::player_interfaces::local::Local;
use tokio::task::JoinSet;
use std::sync::Arc;
use tokio::time::Duration;
use tokio::sync::Mutex;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Service
{
    games: Arc<Mutex<JoinSet<Result<GameEnd, PlayerInterfaceManagerError>>>>,
}

impl Service {
    pub fn new() -> Self {
        Self {
            games: Arc::new(Mutex::new(JoinSet::new())),
        }
    }
}

impl GameManagerService for Service {
    async fn new_game(
        &self,
    ) -> Result<(), ()> {
        let binary_path = Path::new("/Users/guilhem/Repositories/coding-kelps/gomokurs/.debug/gomocku");

        let black = Arc::new(Local::new(binary_path).await.unwrap());
        let white = Arc::new(Local::new(binary_path).await.unwrap());

        let gomoku = BoardStateManager::new(BoardSize{ x: 20, y: 20 });
        let game = GameManager::new(black.clone(), white.clone(), gomoku, Duration::from_secs(30), Duration::from_secs(180));

        let mut players_interface = PlayerInterfacesManager::new();

        let mut games = self.games.lock().await;
        games.spawn(async move {
            tracing::info!("created game!");

            let game_end = players_interface.run(black, white, game).await?;

            tracing::info!("game ended: {}", game_end);

            Ok(game_end)
        });

        Ok(())
    }
}
