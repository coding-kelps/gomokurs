use gomokurs::{domain::game::ports::PlayerClient, outbound::LocalProgram};
use std::path::Path;

#[tokio::main]
async fn main() {
    let binary = Path::new("./.debug/respondToAnythingWith3sDelay");

    let mut pipe = match LocalProgram::new(binary).await {
        Ok(p) => p,
        Err(e) => return eprintln!("{}", e),
    };

    match pipe.request_begin().await {
        Ok(p) => println!("player position: {}", p),
        Err(e) => eprintln!("error {}", e),
    }
}
