use gomokurs_cli::configuration::{Configuration, PlayerConfig};
use gomokurs_game_engine::adapters::player_interfaces::local::Local;
use gomokurs_game_engine::domain::board_state_manager::{BoardStateManager, models::BoardSize};
use gomokurs_game_engine::domain::game_manager::GameManager;
use gomokurs_game_engine::domain::player_interfaces_manager::{PlayerInterfacesManager, PlayerInterfacesManagerService};
use std::sync::Arc;
use tokio::time::Duration;

static HELP: &str = "\
A CLI tool to launch quick gomoku's game\n\
\n\
Usage: gomokurs [OPTIONS] [BLACK PLAYER CONFIGURATION] [WHITE_PLAYER CONFIGURATION]\n\
\n\
Player Configurations:\n\
\tlocal\tA configuration for a local AI program\n\
\tconfiguration:\n\
\t\t--binary <PATH> The path to the AI program binary\n\
\n\
Options:\n\
\t\t--log-level <LEVEL>\tThe logging level of the client (DEBUG, INFO, ERROR...)\n\
";


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Configuration::try_from(std::env::args()) {
        Ok(cfg) => {
            let subscriber = tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).finish();
            let _ = tracing::subscriber::set_global_default(subscriber);

            let black_player = Arc::new(match cfg.black_player {
                PlayerConfig::Local(black_cfg) => Local::new(&black_cfg.binary).await?,
            });
            let white_player = Arc::new(match cfg.white_player {
                PlayerConfig::Local(white_cfg) => Local::new(&white_cfg.binary).await?,
            });

            let gomoku = BoardStateManager::new(BoardSize{ x: 20, y: 20 });

            let game = GameManager::new(
                black_player.clone(),
                white_player.clone(), 
                gomoku, Duration::from_secs(30),
                Duration::from_secs(180),
            );
            let mut players_interface = PlayerInterfacesManager::new();
            let game_end = players_interface.run(black_player, white_player, game).await?;

            tracing::info!("{}!", game_end);

            Ok(())
        },
        Err(e) => {
            eprintln!("{}\n{}", e, HELP);

            Ok(())
        }
    }
}