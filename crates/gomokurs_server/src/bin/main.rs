use gomokurs_server::adapters::http::{HttpServer, HttpServerConfig};
use gomokurs_server::domain::game_manager::service::Service;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG).finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
    
    let game_manager_service = Service::new();
    
    let server_config = HttpServerConfig {
        port: "7777",
    };
    let http_server = HttpServer::new(game_manager_service, server_config).await
        .expect("http server initialization error");
    let _ = http_server.run().await;

    Ok(())
}