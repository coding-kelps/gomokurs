use std::path::Path;
use gomokurs::adapters::clients::Local;
use gomokurs::domain::game::ports::GameService;
use gomokurs::domain::game::Game;

#[tokio::main]
async fn main() {
    let binary = Path::new("./.debug/gomocku");

    let local_1 = Local::new(binary).await.unwrap();
    let local_2 = Local::new(binary).await.unwrap();

    let mut game = Game::new(local_1, local_2, 20);
    let _ = game.play().await;
}
