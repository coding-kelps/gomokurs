use crate::domain::game::ports::{GameService, PlayerNotifier};
use crate::domain::game::models::*;
use crate::domain::gomoku::models::GameEnd;
use crate::domain::gomoku::GomokuService;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Service<N, G>
where
    N: PlayerNotifier,
    G: GomokuService,
{
    black_player: Player<N>,
    white_player: Player<N>,
    gomoku: G,
}

impl<N, G> Service<N, G>
where
    N: PlayerNotifier,
    G: GomokuService,
{
    pub fn new(
        black_player_notifier: Arc<N>,
        white_player_notifier: Arc<N>,
        gomoku: G,
    ) -> Self {
        Self {
            black_player: Player{ color: PlayerColor::Black, ready: false, description: None, notifier: black_player_notifier },
            white_player: Player{ color: PlayerColor::White, ready: false, description: None, notifier: white_player_notifier },
            gomoku,
        }
    }
}

impl<N, G> GameService for Service<N, G>
where
    N: PlayerNotifier,
    G: GomokuService,
{
    async fn init_game(
        &self,
    ) -> Result<(), Error>
    {
        let size = self.gomoku.get_board_size().await;

        self.black_player.notifier
            .notify_start(size.x)
            .await
            .map_err(|error| Error::NotifyError { error, color: self.black_player.color })?;
        self.white_player.notifier
            .notify_start(size.x)
            .await
            .map_err(|error| Error::NotifyError { error, color: self.white_player.color })?;

        self.black_player.notifier
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
            player.notifier
                .notify_error("player has already declared to be ready")
                .await
                .map_err(|error| Error::NotifyError { error, color: player.color })?;
        } else {
            player.ready = true;
        }

        Ok(())
    }

    async fn handle_play(
        &mut self,
        color: PlayerColor,
        position: Position,
    ) -> Result<Option<GameEnd>, Error>
    {         
        let (player, opponent_player) = match color {
            PlayerColor::Black => (&self.black_player, &self.white_player),
            PlayerColor::White => (&self.white_player, &self.black_player),
        };

        if !player.ready {
            player.notifier
                .notify_error("player has already declared to be ready")
                .await
                .map_err(|error| Error::NotifyError { error, color: player.color })?;
        } else {
            match self.gomoku.play_move(player.color, position).await {
                Ok(res) => {
                    if let Some(end) = res {
                        return Ok(Some(end));
                    } else {
                        opponent_player.notifier.notify_turn(position)
                            .await
                            .map_err(|error| Error::NotifyError { error, color: opponent_player.color })?;
                    }
                },
                Err(e) => {
                    player.notifier
                        .notify_error(&e.to_string()) // There is surely a proper way to handle this
                        .await
                        .map_err(|error| Error::NotifyError { error, color: player.color })?;
                },
            }
        }

        Ok(None)
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
