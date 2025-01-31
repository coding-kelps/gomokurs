//! An implementation of the game manager service.
//! 
//! This module provides an implementation of the `GameManagerService` interface
//! for managing a game with black and white players. The service includes game
//! initialization, handling player actions, and managing game timers.

use crate::domain::game_manager::ports::{GameManagerService, PlayerNotifier};
use crate::domain::game_manager::models::*;
use crate::domain::board_state_manager::models::GameEnd;
use crate::domain::board_state_manager::BoardStateManagerService;
use std::sync::Arc;
use tokio::time::Duration;

#[derive(Debug, Clone)]
pub struct Service<N, B>
where
    N: PlayerNotifier,
    B: BoardStateManagerService,
{
    black_player: Player<N>,
    white_player: Player<N>,
    board: B,
}

impl<N, B> Service<N, B>
where
    N: PlayerNotifier,
    B: BoardStateManagerService,
{
    /// Creates a new `Service` instance.
    /// 
    /// # Arguments
    /// * `black_player_notifier` - An `Arc` containing the notifier for the
    /// black player.
    /// * `white_player_notifier` - An `Arc` containing the notifier for the
    /// white player.
    /// * `board` - A board state manager implementing
    /// `BoardStateManagerService`.
    /// * `turn_duration` - The maximum duration of a single player's turn.
    /// * `match_duration` - The maximum duration to the total time allocated
    /// for all of a single player's turns.
    pub fn new(
        black_player_notifier: Arc<N>,
        white_player_notifier: Arc<N>,
        board: B,
        turn_duration: Duration,
        match_duration: Duration,
    ) -> Self {
        Self {
            black_player: Player::new(PlayerColor::Black, black_player_notifier, turn_duration, match_duration),
            white_player: Player::new(PlayerColor::White, white_player_notifier, turn_duration, match_duration),
            board,
        }
    }
}

impl<N, B> GameManagerService for Service<N, B>
where
    N: PlayerNotifier,
    B: BoardStateManagerService,
{
    async fn init_game(
        &self,
    ) -> Result<(), Error>
    {
        let size = self.board.get_size().await;

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

    async fn run_timers(
        &self,
    ) -> Result<GameEnd, Error>
    {
        tokio::select! {
            _ = self.black_player.timer.run(false) => {
                Ok(GameEnd::Win(PlayerColor::White))
            },
            _ = self.white_player.timer.run(true) => {
                Ok(GameEnd::Win(PlayerColor::Black))
            },
        }
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
            match self.board.play_move(player.color, position).await {
                Ok(res) => {
                    if let Some(end) = res {
                        player.notifier.notify_end()
                            .await
                            .map_err(|error| Error::NotifyError { error, color: player.color })?;
                        opponent_player.notifier.notify_end()
                            .await
                            .map_err(|error| Error::NotifyError { error, color: opponent_player.color })?;

                        return Ok(Some(end));
                    } else {
                        opponent_player.notifier.notify_turn(position)
                            .await
                            .map_err(|error| Error::NotifyError { error, color: opponent_player.color })?;
                        
                        player.timer.pause().await;
                        opponent_player.timer.pause().await;
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

    async fn handle_unknown(
        &self,
        color: PlayerColor,
        content: String,
    ) -> Result<(), Error> {
        tracing::error!("{} send unknown error: \"{}\"", color, content);

        Ok(())
    }

    async fn handle_error(
        &self,
        color: PlayerColor,
        content: String,
    ) -> Result<(), Error> {    
        tracing::error!("{} send error: \"{}\"", color, content);

        Ok(())
    }

    async fn handle_message(
        &self,
        color: PlayerColor,
        content: String,
    ) -> Result<(), Error> {
        tracing::info!("{} send message: \"{}\"", color, content);
  
        Ok(())
    }

    async fn handle_debug(
        &self,
        color: PlayerColor,
        content: String,
    ) -> Result<(), Error> {    
        tracing::debug!("{} send debug: \"{}\"", color, content);

        Ok(())
    }

    async fn handle_suggestion(
        &self,
        color: PlayerColor,
        position: Position,
    ) -> Result<(), Error> {
        tracing::info!("{} send suggestion: \"{}\"", color, position);

        Ok(())
    }
}
