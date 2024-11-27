use gomokurs::{domain::game_manager::{models::{GameEnd, PlayRequest}, ports::GameService, Service}, outbound::LocalProgram};
use std::path::Path;

#[tokio::main]
async fn main() {
    let binary = Path::new("./.debug/gomocku");

    let local1 = match LocalProgram::new(binary).await {
        Ok(l) => l,
        Err(e) => return eprintln!("{}", e),
    };

    let local2 = match LocalProgram::new(binary).await {
        Ok(l) => l,
        Err(e) => return eprintln!("{}", e),
    };

    let mut game = Service::new(local1, local2, 20);

    match game.play(&PlayRequest{}).await {
        Ok(end) => match end {
            GameEnd::Win(p) => println!("{} won!", p ),
            GameEnd::Draw => println!("game resulted in a draw!")
        },
        Err(e) => return eprintln!("{}", e),
    };
}
