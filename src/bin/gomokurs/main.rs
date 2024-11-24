use gomokurs::{domain::game::Service, outbound::LocalProgram};
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

    Service::new(local1, local2, 20);
}
