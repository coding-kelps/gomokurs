use crate::domain::game_manager::ports::GameManagerService;
use crate::domain::game::ports::{PlayerClient, GameService};
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

impl<PC, GS> GameManagerService<PC, GS> for Service
where
    PC: PlayerClient,
    GS: GameService,
{
    async fn run(
        &mut self,
        black_client: Arc<PC>,
        white_client: Arc<PC>,
        mut game: GS,
    ) -> Result<GameEnd, Error>
    {
        let (actions_tx_black, mut actions_rx) = channel::<(PlayerColor, PlayerAction)>(100);
        let actions_tx_white = actions_tx_black.clone();
        
        let mut listeners = JoinSet::new();
        listeners.spawn(async move { black_client.listen(actions_tx_black, PlayerColor::Black).await });
        listeners.spawn(async move { white_client.listen(actions_tx_white, PlayerColor::White).await });
        
        game.init_game().await?;
        
        while let Some((color, action)) = actions_rx.recv().await {            
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
        }

        Err(Error::ChannelClosed)
    }
}
