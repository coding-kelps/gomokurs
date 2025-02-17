use gomokurs_coordinator::adapters::player_interfaces::local::{LocalPlayerInterface, CreateLocalPlayerInterfaceError};
use gomokurs_coordinator::adapters::player_interfaces::tcp::{CreateTcpPlayerInterfaceConfiguration, TcpPlayerInterface, CreateTcpPlayerInterfaceError};
use gomokurs_coordinator::domain::coordinator::ports::PlayerInterface;
use gomokurs_coordinator::domain::coordinator::models::*;
use tokio::sync::mpsc::Sender;
use tokio::net::{TcpStream, TcpListener};
use crate::configuration::player_configuration::{PlayerConfiguration, ProtocolConfiguration, TcpConfiguration};
use thiserror::Error;

pub enum PlayerInterfaceOption {
    Local(LocalPlayerInterface),
    Tcp(TcpPlayerInterface),
}

#[derive(Debug, Error)]
pub enum CreatePlayerInterfaceFromCfgError {
    #[error(transparent)]
    Local(#[from] CreateLocalPlayerInterfaceError),
    #[error(transparent)]
    Tcp(#[from] CreateTcpPlayerInterfaceError),
    #[error("tcp connection error: `{0}`")]
    CreateClientError(#[from] tokio::io::Error),
}

pub async fn create_player_interface_from_cfg(cfg: PlayerConfiguration) -> Result<PlayerInterfaceOption, CreatePlayerInterfaceFromCfgError>
{
    match cfg.protocol {
        ProtocolConfiguration::Stdio(stdio_cfg) => {
            Ok(PlayerInterfaceOption::Local(LocalPlayerInterface::new(&stdio_cfg.binary, stdio_cfg.args).await?))
        },
        ProtocolConfiguration::Tcp(tcp_cfg) => {
            match tcp_cfg {
                TcpConfiguration::Active(active_tcp_cfg) => {
                    let stream = TcpStream::connect(active_tcp_cfg.address).await?;

                    let tcp_interface_cfg = CreateTcpPlayerInterfaceConfiguration{
                        stream: stream,
                    };

                    Ok(PlayerInterfaceOption::Tcp(TcpPlayerInterface::new(tcp_interface_cfg).await?))

                },
                TcpConfiguration::Passive(passive_tcp_cfg) => {
                    let listener = TcpListener::bind(passive_tcp_cfg.address).await?;

                    let (stream, _) = listener.accept().await?;

                    let tcp_interface_cfg = CreateTcpPlayerInterfaceConfiguration{
                        stream: stream,
                    };

                    Ok(PlayerInterfaceOption::Tcp(TcpPlayerInterface::new(tcp_interface_cfg).await?))
                },
            }
        }
    }
}

// Implement the traits by delegating to the wrapped type
impl PlayerInterface for PlayerInterfaceOption {
    async fn listen(
        &self,
        color: PlayerColor,
        tx: Sender<(PlayerColor, PlayerAction)>,
    ) -> Result<(), ListenError> {
        match self {
            PlayerInterfaceOption::Local(local) => local.listen(color, tx).await,
            PlayerInterfaceOption::Tcp(tcp)     => tcp.listen(color, tx).await,
        }
    }

    async fn notify_start(
        &self,
        size: u8,
    ) -> Result<(), NotifyError> {
        match self {
            PlayerInterfaceOption::Local(local) => local.notify_start(size).await,
            PlayerInterfaceOption::Tcp(tcp)     => tcp.notify_start(size).await,
        }
    }

    async fn notify_restart(
        &self,
    ) -> Result<(), NotifyError> {
        match self {
            PlayerInterfaceOption::Local(local) => local.notify_restart().await,
            PlayerInterfaceOption::Tcp(tcp)     => tcp.notify_restart().await,
        }
    }

    async fn notify_turn(
        &self,
        position: Position,
    ) -> Result<(), NotifyError> {
        match self {
            PlayerInterfaceOption::Local(local) => local.notify_turn(position).await,
            PlayerInterfaceOption::Tcp(tcp)     => tcp.notify_turn(position).await,
        }
    }

    async fn notify_begin(
        &self,
    ) -> Result<(), NotifyError> {
        match self {
            PlayerInterfaceOption::Local(local) => local.notify_begin().await,
            PlayerInterfaceOption::Tcp(tcp)     => tcp.notify_begin().await,
        }
    }

    async fn notify_board(
        &self,
        turns: Vec<RelativeTurn>,
    ) -> Result<(), NotifyError> {
        match self {
            PlayerInterfaceOption::Local(local) => local.notify_board(turns).await,
            PlayerInterfaceOption::Tcp(tcp)     => tcp.notify_board(turns).await,
        }
    }

    async fn notify_info(
        &self,
        info: Information,
    ) -> Result<(), NotifyError> {
        match self {
            PlayerInterfaceOption::Local(local) => local.notify_info(info).await,
            PlayerInterfaceOption::Tcp(tcp)     => tcp.notify_info(info).await,
        }
    }

    async fn notify_result(
        &self,
        result: RelativeGameEnd,
    ) -> Result<(), NotifyError> {
        match self {
            PlayerInterfaceOption::Local(local) => local.notify_result(result).await,
            PlayerInterfaceOption::Tcp(tcp)     => tcp.notify_result(result).await,
        }
    }

    async fn notify_end(
        &self,
    ) -> Result<(), NotifyError> {
        match self {
            PlayerInterfaceOption::Local(local) => local.notify_end().await,
            PlayerInterfaceOption::Tcp(tcp)     => tcp.notify_end().await,
        }
    }

    async fn notify_about(
        &self,
    ) -> Result<(), NotifyError> {
        match self {
            PlayerInterfaceOption::Local(local) => local.notify_about().await,
            PlayerInterfaceOption::Tcp(tcp)     => tcp.notify_about().await,
        }
    }

    async fn notify_unknown(
        &self,
        content: &str,
    ) -> Result<(), NotifyError> {
        match self {
            PlayerInterfaceOption::Local(local) => local.notify_unknown(content).await,
            PlayerInterfaceOption::Tcp(tcp)     => tcp.notify_unknown(content).await,
        }
    }

    async fn notify_error(
        &self,
        content: &str,
    ) -> Result<(), NotifyError> {
        match self {
            PlayerInterfaceOption::Local(local) => local.notify_error(content).await,
            PlayerInterfaceOption::Tcp(tcp)     => tcp.notify_error(content).await,
        }
    }
}
