use std::path::Path;
use gomokurs::adapters::Local;

#[tokio::main]
async fn main() {
    let binary = Path::new("./.debug/gomocku");

    let _local_1 = Local::new(binary).await.unwrap();
    let _local_2 = Local::new(binary).await.unwrap();

    
}
