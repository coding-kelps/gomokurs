use crate::domain::game_manager::ports::{GameManagerService, PlayerListener};
use crate::domain::game::ports::{GameService, PlayerNotifier};
use tokio::task::JoinSet;
use tokio::sync::mpsc::channel;
use crate::domain::game::models::{PlayerColor, PlayerAction};
use crate::domain::gomoku::models::GameEnd;
use crate::domain::game_manager::models::Error;
use std::sync::Arc;

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

impl<C, G> GameManagerService<C, G> for Service
where
    C: PlayerListener + PlayerNotifier,
    G: GameService,
{
    async fn run(
        &mut self,
        black_client: Arc<C>,
        white_client: Arc<C>,
        mut game: G,
    ) -> Result<GameEnd, Error>
    {
        let (actions_tx_black, mut actions_rx) = channel::<(PlayerColor, PlayerAction)>(100);
        let actions_tx_white = actions_tx_black.clone();
        
        let mut listeners = JoinSet::new();
        listeners.spawn(async move { black_client.listen(actions_tx_black, PlayerColor::Black).await });
        listeners.spawn(async move { white_client.listen(actions_tx_white, PlayerColor::White).await });
        
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
