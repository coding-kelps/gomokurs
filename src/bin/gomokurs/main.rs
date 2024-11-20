use gomokurs::outbound::NamedPipe;
use std::path::Path;

#[tokio::main]
async fn main() {
    let binary = Path::new("/home/ubuntu/Repositories/coding-kelps/gomokurs/.debug/main");

    let mut pipe = match NamedPipe::new(binary).await {
        Ok(p) => p,
        Err(e) => return eprintln!("{}", e),
    };
}
