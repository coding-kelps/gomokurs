use gomokurs_cli::{configuration::{cli::Cli, player_configuration::PlayerConfiguration}, player_interface::create_player_interface_from_cfg};
use clap::Parser;
use gomokurs_game_engine::domain::board_state_manager::{BoardStateManager, models::BoardSize};
use gomokurs_game_engine::domain::game_manager::GameManager;
use gomokurs_game_engine::domain::player_interfaces_manager::{PlayerInterfacesManager, PlayerInterfacesManagerService};
use std::sync::Arc;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let cli = match Cli::try_parse() {
        Ok(cli) => {
            cli
        },
        Err(e) => {
            println!("error: {}", e);

            return
        },
    };

    let subscriber = tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).finish();
    let _ = tracing::subscriber::set_global_default(subscriber);

    let black_cfg = match PlayerConfiguration::new(&cli.black_file) {
        Ok(cfg) => cfg,
        Err(e) => {
            tracing::error!("{}", e);

            return
        }
    };

    let white_cfg = match PlayerConfiguration::new(&cli.white_file) {
        Ok(cfg) => cfg,
        Err(e) => {
            tracing::error!("{}", e);

            return
        }
    };

    let black_player = match create_player_interface_from_cfg(black_cfg).await {
        Ok(interface) => Arc::new(interface),
        Err(e) => {
            tracing::error!("{}", e);

            return
        }
    };
    let white_player = match create_player_interface_from_cfg(white_cfg).await {
        Ok(interface) => Arc::new(interface),
        Err(e) => {
            tracing::error!("{}", e);

            return
        }
    };

    let gomoku = BoardStateManager::new(BoardSize{ x: 20, y: 20 });

    let game = GameManager::new(
        black_player.clone(),
        white_player.clone(), 
        gomoku, Duration::from_secs(30),
        Duration::from_secs(180),
    );
    let mut players_interface = PlayerInterfacesManager::new();
    let game_end = players_interface.run(black_player, white_player, game).await.unwrap();

    tracing::info!("{}!", game_end);
}
