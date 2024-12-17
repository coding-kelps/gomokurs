//! An implementation of the player interfaces manager service.

use crate::domain::player_interfaces_manager::ports::{PlayerInterfacesManagerService, PlayerListener};
use crate::domain::game_manager::ports::{GameManagerService, PlayerNotifier};
use tokio::task::JoinSet;
use tokio::sync::mpsc::channel;
use crate::domain::game_manager::models::{PlayerColor, PlayerAction};
use crate::domain::board_state_manager::models::GameEnd;
use crate::domain::player_interfaces_manager::models::Error;
use std::sync::Arc;

/// An implementation of the player interfaces manager service.
#[derive(Debug, Clone)]
pub struct Service
{
}

impl Service
{
    pub fn new(
    ) -> Self {
        Self {}
    }
}

impl<I, G> PlayerInterfacesManagerService<I, G> for Service
where
    I: PlayerListener + PlayerNotifier,
    G: GameManagerService,
{
    /// Run a game from a given game_manager, coordinating both white and black
    /// players interface concurrent inputs for the game manager, eventually
    /// returns the game ending.
    /// 
    /// # Arguments
    /// 
    /// * `black_interface` - An Arc to a player interface of what will be
    /// considered the black player during the game.
    /// * `white_interface` - An Arc to a player interface of what will be
    /// considered the white player during the game.
    /// * `game_manager` - A game manager
    async fn run(
        &mut self,
        black_interface: Arc<I>,
        white_interface: Arc<I>,
        mut game: G,
    ) -> Result<GameEnd, Error>
    {
        let (actions_tx, mut actions_rx) = channel::<(PlayerColor, PlayerAction)>(100);
        let (actions_tx_black, actions_tx_white) = (actions_tx.clone(), actions_tx.clone());
        
        // Start listening to players.
        let mut listeners = JoinSet::new();
        listeners.spawn(async move { black_interface.listen(PlayerColor::Black, actions_tx_black).await });
        listeners.spawn(async move { white_interface.listen(PlayerColor::White, actions_tx_white).await });
        
        game.init_game().await?;

        loop {
            tokio::select! {
                Some((color, action)) = actions_rx.recv() => {
                    tracing::debug!("received {:?} from {}", action, color);

                    match action {
                        PlayerAction::Ok => {                    
                            game.handle_ok(color).await?;
                        },
                        PlayerAction::Play(position) => {
                            if let Some(end) = game.handle_play(color, position).await? {
                                return Ok(end);
                            }
                        },
                        PlayerAction::Description(description) => {
                            game.handle_description(color, description).await?;
                        },
                        PlayerAction::Unknown(content) => {
                            game.handle_unknown(color, content).await?;
                        },
                        PlayerAction::Error(content) => {
                            game.handle_error(color, content).await?;
                        },
                        PlayerAction::Message(content) => {
                            game.handle_message(color, content).await?;
                        },
                        PlayerAction::Debug(content) => {
                            game.handle_debug(color, content).await?;
                        },
                        PlayerAction::Suggestion(position) => {
                            game.handle_suggestion(color, position).await?;
                        },
                    }
                },
                res = game.run_timers() => {
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
