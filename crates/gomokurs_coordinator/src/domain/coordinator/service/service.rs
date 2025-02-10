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

/// Represents the Player Interfaces Manager service.
///
/// This service is responsible for:
/// - Managing player listener adapters.
/// - Routing player input to the appropriate `GameManager` handlers.
/// - Providing a bridge between players and the game logic.
#[derive(Debug, Clone)]
pub struct Service<G, I>
where
    G: GameEngineService,
    I: PlayerInterface,
{
    pub game: G,
    pub black: Player<I>,
    pub white: Player<I>,
}

impl<G, I> Service<G, I>
where
    G: GameEngineService,
    I: PlayerInterface,
{
    /// Creates a new instance of the Player Interfaces Manager service.
    pub fn new(
        game: G,
        black: Arc<I>,
        white: Arc<I>,
    ) -> Self {
        Self {
            game: game,
            black: Player::new(PlayerColor::Black, black),
            white: Player::new(PlayerColor::White, white),
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
                                return Ok(end);
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
