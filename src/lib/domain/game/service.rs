use crate::domain::game::ports::{GameService, PlayerClient};
use crate::domain::game::models::*;
use crate::domain::gomoku::models::GameEnd;
use crate::domain::gomoku::GomokuService;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Service<PC, GKS>
where
    PC: PlayerClient,
    GKS: GomokuService,
{
    black_player: Player<PC>,
    white_player: Player<PC>,
    gomoku: GKS,
}

impl<PC, GKS> Service<PC, GKS>
where
    PC: PlayerClient,
    GKS: GomokuService,
{
    pub fn new(
        black_player_client: Arc<PC>,
        white_player_client: Arc<PC>,
        gomoku: GKS,
    ) -> Self {
        Self {
            black_player: Player{ color: PlayerColor::Black, ready: false, description: None, client: black_player_client },
            white_player: Player{ color: PlayerColor::White, ready: false, description: None, client: white_player_client },
            gomoku,
        }
}
}

impl<PC, GKS> GameService for Service<PC, GKS>
where
    PC: PlayerClient,
    GKS: GomokuService,
{
    async fn init_game(
        &self,
    ) -> Result<(), Error>
    {
        self.black_player.client
            .notify_start(20)
            .await
            .map_err(|error| Error::NotifyError { error, color: self.black_player.color })?;
        self.white_player.client
            .notify_start(20)
            .await
            .map_err(|error| Error::NotifyError { error, color: self.white_player.color })?;

        self.black_player.client
            .notify_begin()
            .await
            .map_err(|error| Error::NotifyError { error, color: self.black_player.color })?;

        Ok(())
    }

    async fn handle_ok(
        &mut self,
        color: PlayerColor,
    ) -> Result<(), Error>
    {
        let player = match color {
            PlayerColor::Black => &mut self.black_player,
            PlayerColor::White => &mut self.white_player,
        };

        if player.ready {
            player.client
                .notify_error("player has already declared to be ready")
                .await
                .map_err(|error| Error::NotifyError { error, color: player.color })?;

            // Send Err
        } else {
            player.ready = true;

            // Send Ok
        }

        Ok(())
    }

    async fn handle_play(
        &mut self,
        color: PlayerColor,
        position: Position,
    ) -> Result<(), Error>
    {         
        let (player, opponent_player) = match color {
            PlayerColor::Black => (&self.black_player, &self.white_player),
            PlayerColor::White => (&self.white_player, &self.black_player),
        };

        if !player.ready {
            player.client
                .notify_error("player has already declared to be ready")
                .await
                .map_err(|error| Error::NotifyError { error, color: player.color })?;

            // Send Err
        } else {
            if let Some(end) = self.gomoku.play_move(player.color, position).await.unwrap() /* Handle error */ {
                if let GameEnd::Win(winning_player) = end {
                    println!("{} won!", winning_player);
                }
            } else {
                opponent_player.client.notify_turn(position).await; // Handle
            }

            // Send Ok
        }

        Ok(())
    }

    async fn handle_description(
        &mut self,
        color: PlayerColor,
        description: PlayerDescription,
    ) -> Result<(), Error> {
        let player = match color {
            PlayerColor::Black => &mut self.black_player,
            PlayerColor::White => &mut self.white_player,
        };

        player.description = Some(description);

        Ok(())
    }

    #[allow(unused_variables)]
    async fn handle_unknown(
        &self,
        color: PlayerColor,
        content: String,
    ) -> Result<(), Error> {    
        Ok(())
    }

    #[allow(unused_variables)]
    async fn handle_error(
        &self,
        color: PlayerColor,
        content: String,
    ) -> Result<(), Error> {    
        Ok(())
    }

    #[allow(unused_variables)]
    async fn handle_message(
        &self,
        color: PlayerColor,
        content: String,
    ) -> Result<(), Error> {    
        Ok(())
    }

    #[allow(unused_variables)]
    async fn handle_debug(
        &self,
        color: PlayerColor,
        content: String,
    ) -> Result<(), Error> {    
        Ok(())
    }

    #[allow(unused_variables)]
    async fn handle_suggestion(
        &self,
        color: PlayerColor,
        position: Position,
    ) -> Result<(), Error> {
        Ok(())
    }
}
