use std::path::Path;
use gomokurs::adapters::clients::Local;
use gomokurs::domain::board_state_manager::{BoardStateManager, models::BoardSize};
use gomokurs::domain::game_manager::GameManager;
use gomokurs::domain::players_interface::{PlayersInterface, PlayersInterfaceService};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).finish();
    let _ = tracing::subscriber::set_global_default(subscriber);

    let binary = Path::new("./.debug/gomocku");

    let local_1 = Arc::new(Local::new(binary).await.unwrap());
    let local_2 = Arc::new(Local::new(binary).await.unwrap());
    let gomoku = BoardStateManager::new(BoardSize{ x: 20, y: 20 });

    let game = GameManager::new(local_1.clone(), local_2.clone(), gomoku);

    let mut players_interface = PlayersInterface::new();

    let _ = players_interface.run(local_1, local_2, game).await;
}
