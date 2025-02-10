//! Implementation of the Player Interfaces Manager service.
//!
//! This service manages player listener adapters, linking them to the
//! corresponding `GameManager` handlers.

use crate::domain::coordinator::ports::{GameEngineService, CoordinatorService, PlayerInterface};
use crate::domain::coordinator::service::player::Player;
use tokio::task::JoinSet;
use tokio::sync::mpsc::channel;
use crate::domain::coordinator::models::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CreateCoordinatorConfiguration<G, I>
where
    G: GameEngineService,
    I: PlayerInterface,
{
    pub game_engine: G,
    pub black_player_interface: Arc<I>,
    pub white_player_interface: Arc<I>,
    pub game_mode: Mode,
}

#[derive(Debug, Clone)]
pub struct Service<G, I>
where
    G: GameEngineService,
    I: PlayerInterface,
{
    pub game: G,
    pub black: Player<I>,
    pub white: Player<I>,
    pub mode: Mode,
}

impl<G, I> Service<G, I>
where
    G: GameEngineService,
    I: PlayerInterface,
{
    /// Creates a new instance of the Player Interfaces Manager service.
    pub fn new(
        cfg: CreateCoordinatorConfiguration<G, I>,
    ) -> Self {
        Self {
            game: cfg.game_engine,
            black: Player::new(PlayerColor::Black, cfg.black_player_interface),
            white: Player::new(PlayerColor::White, cfg.white_player_interface),
            mode: cfg.game_mode,
        }
    }

    pub async fn start_game(
        &self
    ) -> Result<(), Error>
    {
        let size = self.game.get_board_size().await;

        self.black.interface
            .notify_start(size.x)
            .await
            .map_err(|error| Error::NotifyError { error, color: self.black.color })?;
        self.white.interface
            .notify_start(size.x)
            .await
            .map_err(|error| Error::NotifyError { error, color: self.white.color })?;

        self.black.interface
            .notify_begin()
            .await
            .map_err(|error| Error::NotifyError { error, color: self.black.color })?;

        Ok(())
    }

    pub async fn end_game(
        &self
    ) -> Result<(), Error>
    {
        self.black.interface.notify_end()
            .await
            .map_err(|error| Error::NotifyError { error, color: self.black.color })?;
    
        self.white.interface.notify_end()
            .await
            .map_err(|error| Error::NotifyError { error, color: self.white.color })?;

        Ok(())
    }

    pub async fn restart_game(
        &mut self
    ) -> Result<(), Error>
    {
        tracing::debug!("loop mode - restart game");

        self.game.reset().await?;

        self.black.interface.notify_restart().await
            .map_err(|error| Error::NotifyError { error, color: self.black.color })?;

        self.white.interface.notify_restart().await
            .map_err(|error| Error::NotifyError { error, color: self.white.color })?;

        self.black.interface.notify_begin().await
            .map_err(|error| Error::NotifyError { error, color: self.black.color })?;

        Ok(())
    }
}

impl<G, I> CoordinatorService<G, I> for Service<G, I>
where
    G: GameEngineService,
    I: PlayerInterface,
{
    async fn run(
        &mut self,
    ) -> Result<GameEnd, Error>
    {
        let (actions_tx, mut actions_rx) = channel::<(PlayerColor, PlayerAction)>(100);
        let (actions_tx_black, actions_tx_white) = (actions_tx.clone(), actions_tx.clone());
        
        // Start listening to players.
        let mut listeners = JoinSet::new();

        let black_interface = self.black.interface.clone();
        listeners.spawn(async move { black_interface.listen(PlayerColor::Black, actions_tx_black).await });

        let white_interface = self.white.interface.clone();
        listeners.spawn(async move { white_interface.listen(PlayerColor::White, actions_tx_white).await });
        
        self.start_game().await?;

        loop {
            tokio::select! {
                Some((color, action)) = actions_rx.recv() => {
                    tracing::debug!("received {:?} from {}", action, color);

                    match action {
                        PlayerAction::Ready => {                    
                            self.handle_ready(color).await?;
                        },
                        PlayerAction::Play(position) => {
                            if let Some(end) = self.handle_play(color, position).await? {
                                match self.mode {
                                    Mode::SingleGame => {
                                        self.end_game().await?;

                                        return Ok(end)
                                    },
                                    Mode::Loop => self.restart_game().await?,
                                }
                            }
                        },
                        PlayerAction::Metadata(metadata) => {
                            self.handle_metadata(color, metadata).await?;
                        },
                        PlayerAction::Unknown(content) => {
                            self.handle_unknown(color, content).await?;
                        },
                        PlayerAction::Error(content) => {
                            self.handle_error(color, content).await?;
                        },
                        PlayerAction::Message(content) => {
                            self.handle_message(color, content).await?;
                        },
                        PlayerAction::Debug(content) => {
                            self.handle_debug(color, content).await?;
                        },
                        PlayerAction::Suggestion(position) => {
                            self.handle_suggestion(color, position).await?;
                        },
                    }
                },
                res = self.game.run_timers() => {
                    match res {
                        Ok(end) => {
                            return Ok(end)
                        },
                        Err(e) => return Err(e.into()),
                    }
                },
                Some(res) = listeners.join_next() => {
                    match res {
                        Err(e) => return Err(e.into()),
                        Ok(listener_res) => match listener_res {
                            Err(e)=> return Err(e.into()),
                            Ok(_) => continue,
                        }
                    }
                },
                else => {
                    return Err(Error::ChannelClosed);
                }
            }
        }
    }
}
