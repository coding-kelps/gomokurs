use gomokurs_cli::{configuration::{cli::Cli, player_configuration::PlayerConfiguration}, player_interface::create_player_interface_from_cfg};
use clap::Parser;
use gomokurs_game_engine::domain::board_state_manager::{BoardStateManager, models::BoardSize};
use gomokurs_game_engine::domain::game_manager::GameManager;
use gomokurs_coordinator::domain::player_interfaces_manager::{PlayerInterfacesManager, PlayerInterfacesManagerService};
use std::{str::FromStr, sync::Arc};
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

    let log_level = match tracing::Level::from_str(&cli.log_level) {
        Ok(lvl) => lvl,
        Err(e) => {
            println!("error: {}", e);

            return
        }
    };

    let subscriber = tracing_subscriber::fmt().with_max_level(log_level).finish();
    let _ = tracing::subscriber::set_global_default(subscriber);

    let black_cfg = match PlayerConfiguration::new(&cli.black_file) {
        Ok(cfg) => cfg,
        Err(e) => {
            tracing::error!("failed to read black player configuration file: {}", e);

            return
        }
    };

    let white_cfg = match PlayerConfiguration::new(&cli.white_file) {
        Ok(cfg) => cfg,
        Err(e) => {
            tracing::error!("failed to read white player configuration file: {}", e);

            return
        }
    };

    let black_player = match create_player_interface_from_cfg(black_cfg).await {
        Ok(interface) => Arc::new(interface),
        Err(e) => {
            tracing::error!("failed to create black player interface: {}", e);

            return
        }
    };
    tracing::debug!("created black player interface");

    let white_player = match create_player_interface_from_cfg(white_cfg).await {
        Ok(interface) => Arc::new(interface),
        Err(e) => {
            tracing::error!("failed to create white player interface: {}", e);

            return
        }
    };
    tracing::debug!("created white player interface");

    let gomoku = BoardStateManager::new(BoardSize{ x: 20, y: 20 });

    let game = GameManager::new(
        black_player.clone(),
        white_player.clone(), 
        gomoku,
        Duration::from_secs(cli.turn_duration),
        Duration::from_secs(cli.match_duration),
    );
    let mut players_interface = PlayerInterfacesManager::new();
    let game_end = players_interface.run(black_player, white_player, game).await.unwrap();

    tracing::info!("{}!", game_end);
}
