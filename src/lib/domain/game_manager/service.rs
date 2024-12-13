use crate::domain::game_manager::ports::GameManagerService;
use crate::domain::game::ports::{PlayerClient, GameService};
use tokio::task::JoinSet;
use tokio::sync::mpsc::channel;
use crate::domain::game::models::{PlayerColor, PlayerAction};
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
    ) -> Result<(), ()>
    {
        let (actions_tx_black, mut actions_rx) = channel::<(PlayerColor, PlayerAction)>(100);
        let actions_tx_white = actions_tx_black.clone();
        
        let mut listeners = JoinSet::new();
        listeners.spawn(async move { black_client.listen(actions_tx_black, PlayerColor::Black).await });
        listeners.spawn(async move { white_client.listen(actions_tx_white, PlayerColor::White).await });
        
        let _ = game.init_game().await;
        
        while let Some((color, action)) = actions_rx.recv().await {            
            match action {
                PlayerAction::Ok => {                    
                    let _ = game.handle_ok(color).await;
                },
                PlayerAction::Play(position) => {
                    let _ = game.handle_play(color, position).await;
                },
                PlayerAction::Description(description) => {
                    let _ = game.handle_description(color, description).await;
                },
                PlayerAction::Unknown(content) => {
                    let _ = game.handle_unknown(color, content).await;
                },
                PlayerAction::Error(content) => {
                    let _ = game.handle_error(color, content).await;
                },
                PlayerAction::Message(content) => {
                    let _ = game.handle_message(color, content).await;
                },
                PlayerAction::Debug(content) => {
                    let _ = game.handle_debug(color, content).await;
                },
                PlayerAction::Suggestion(position) => {
                    let _ = game.handle_suggestion(color, position).await;
                },
            }
        }

        Ok(())
    }
}
