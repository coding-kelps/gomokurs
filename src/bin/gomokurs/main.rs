use gomokurs::adapters::LocalProgram;
use std::path::Path;

#[tokio::main]
async fn main() {
    let binary = Path::new("./.debug/gomocku");

    let _ = match LocalProgram::new(binary).await {
        Ok(l) => l,
        Err(e) => return eprintln!("{}", e),
    };

    let _ = match LocalProgram::new(binary).await {
        Ok(l) => l,
        Err(e) => return eprintln!("{}", e),
    };
}
