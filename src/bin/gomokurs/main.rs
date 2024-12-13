use std::path::Path;
use gomokurs::adapters::clients::Local;
use gomokurs::domain::gomoku::Gomoku;
use gomokurs::domain::game::Game;
use gomokurs::domain::game_manager::{GameManager, GameManagerService};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let binary = Path::new("./.debug/gomocku");

    let local_1 = Arc::new(Local::new(binary).await.unwrap());
    let local_2 = Arc::new(Local::new(binary).await.unwrap());
    let gomoku = Gomoku::new(20);

    let game = Game::new(local_1.clone(), local_2.clone(), gomoku);

    let mut game_manager = GameManager::new();

    let _ = game_manager.run(local_1, local_2, game).await;
}
