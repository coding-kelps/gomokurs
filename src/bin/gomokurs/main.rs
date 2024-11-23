use gomokurs::{domain::game::ports::PlayerClient, outbound::LocalProgram};
use std::path::Path;

#[tokio::main]
async fn main() {
    let binary = Path::new("./.debug/gomocku");

    let mut local = match LocalProgram::new(binary).await {
        Ok(l) => l,
        Err(e) => return eprintln!("{}", e),
    };

    match local.request_start(20).await {
        Ok(_) => (),
        Err(e) => eprintln!("error {}", e),
    }

    match local.request_begin().await {
        Ok(p) => println!("player position: {}", p),
        Err(e) => eprintln!("error {}", e),
    }
}
