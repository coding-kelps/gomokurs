use gomokurs_game_engine::{adapters::player_interfaces::{local::{Local, local::CreateLocalProgramError}, tcp::{tcp::{CreateTcpInterfaceConfiguration, CreateTcpInterfaceError}, Tcp}}, domain::{game_manager::ports::PlayerNotifier, player_interfaces_manager::ports::PlayerListener}};
use gomokurs_game_engine::domain::game_manager::models::{PlayerColor, PlayerAction};
use gomokurs_game_engine::domain::player_interfaces_manager::models::ListenError;
use tokio::sync::mpsc::Sender;
use tokio::net::{TcpStream, TcpListener};
use gomokurs_game_engine::domain::board_state_manager::models::Position;
use gomokurs_game_engine::domain::game_manager::models::{RelativeTurn, Information, NotifyError};
use crate::configuration::player_configuration::{PlayerConfiguration, ProtocolConfiguration, TcpConfiguration};
use thiserror::Error;

pub enum PlayerInterface {
    Local(Local),
    Tcp(Tcp),
}

#[derive(Debug, Error)]
pub enum CreatePlayerInterfaceFromCfgError {
    #[error(transparent)]
    Local(#[from] CreateLocalProgramError),
    #[error(transparent)]
    Tcp(#[from] CreateTcpInterfaceError),
    #[error("tcp connection error: `{0}`")]
    CreateClientError(#[from] tokio::io::Error),
}

pub async fn create_player_interface_from_cfg(cfg: PlayerConfiguration) -> Result<PlayerInterface, CreatePlayerInterfaceFromCfgError>
{
    match cfg.protocol {
        ProtocolConfiguration::Stdio(stdio_cfg) => {
            Ok(PlayerInterface::Local(Local::new(&stdio_cfg.binary, stdio_cfg.args).await?))
        },
        ProtocolConfiguration::Tcp(tcp_cfg) => {
            match tcp_cfg {
                TcpConfiguration::Active(active_tcp_cfg) => {
                    let stream = TcpStream::connect(active_tcp_cfg.address).await?;

                    let tcp_interface_cfg = CreateTcpInterfaceConfiguration{
                        stream: stream,
                    };

                    Ok(PlayerInterface::Tcp(Tcp::new(tcp_interface_cfg).await?))

                },
                TcpConfiguration::Passive(passive_tcp_cfg) => {
                    let listener = TcpListener::bind(passive_tcp_cfg.address).await?;

                    let (stream, _) = listener.accept().await?;

                    let tcp_interface_cfg = CreateTcpInterfaceConfiguration{
                        stream: stream,
                    };

                    Ok(PlayerInterface::Tcp(Tcp::new(tcp_interface_cfg).await?))
                },
            }
        }
    }
}

// Implement the traits by delegating to the wrapped type
impl PlayerListener for PlayerInterface {
    async fn listen(
        &self,
        color: PlayerColor,
        tx: Sender<(PlayerColor, PlayerAction)>,
    ) -> Result<(), ListenError> {
        match self {
            PlayerInterface::Local(local) => local.listen(color, tx).await,
            PlayerInterface::Tcp(tcp)     => tcp.listen(color, tx).await,
        }
    }
}

impl PlayerNotifier for PlayerInterface {
    async fn notify_start(
        &self,
        size: u8,
    ) -> Result<(), NotifyError> {
        match self {
            PlayerInterface::Local(local) => local.notify_start(size).await,
            PlayerInterface::Tcp(tcp)     => tcp.notify_start(size).await,
        }
    }

    async fn notify_turn(
        &self,
        position: Position,
    ) -> Result<(), NotifyError> {
        match self {
            PlayerInterface::Local(local) => local.notify_turn(position).await,
            PlayerInterface::Tcp(tcp)     => tcp.notify_turn(position).await,
        }
    }

    async fn notify_begin(
        &self,
    ) -> Result<(), NotifyError> {
        match self {
            PlayerInterface::Local(local) => local.notify_begin().await,
            PlayerInterface::Tcp(tcp)     => tcp.notify_begin().await,
        }
    }

    async fn notify_board(
        &self,
        turns: Vec<RelativeTurn>,
    ) -> Result<(), NotifyError> {
        match self {
            PlayerInterface::Local(local) => local.notify_board(turns).await,
            PlayerInterface::Tcp(tcp)     => tcp.notify_board(turns).await,
        }
    }

    async fn notify_info(
        &self,
        info: Information,
    ) -> Result<(), NotifyError> {
        match self {
            PlayerInterface::Local(local) => local.notify_info(info).await,
            PlayerInterface::Tcp(tcp)     => tcp.notify_info(info).await,
        }
    }

    async fn notify_end(
        &self,
    ) -> Result<(), NotifyError> {
        match self {
            PlayerInterface::Local(local) => local.notify_end().await,
            PlayerInterface::Tcp(tcp)     => tcp.notify_end().await,
        }
    }

    async fn notify_about(
        &self,
    ) -> Result<(), NotifyError> {
        match self {
            PlayerInterface::Local(local) => local.notify_about().await,
            PlayerInterface::Tcp(tcp)     => tcp.notify_about().await,
        }
    }

    async fn notify_unknown(
        &self,
        content: &str,
    ) -> Result<(), NotifyError> {
        match self {
            PlayerInterface::Local(local) => local.notify_unknown(content).await,
            PlayerInterface::Tcp(tcp)     => tcp.notify_unknown(content).await,
        }
    }

    async fn notify_error(
        &self,
        content: &str,
    ) -> Result<(), NotifyError> {
        match self {
            PlayerInterface::Local(local) => local.notify_error(content).await,
            PlayerInterface::Tcp(tcp)     => tcp.notify_error(content).await,
        }
    }
}