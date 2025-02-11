use crate::domain::coordinator::ports::PlayerInterface;
use gomokurs_game_engine::domain::game_engine::ports::GameEngineService;
use crate::domain::coordinator::service::service::Service;
use crate::domain::coordinator::models::*;

impl<G, I> Service<G, I>
where
    G: GameEngineService,
    I: PlayerInterface
{
    pub async fn handle_ready(
        &mut self,
        color: PlayerColor
    ) -> Result<(), Error>
    {
        let player = match color {
            PlayerColor::Black => &mut self.black,
            PlayerColor::White => &mut self.white,
        };

        if player.ready {
            player.interface
                .notify_error("player has already declared to be ready")
                .await
                .map_err(|error| Error::NotifyError { error, color: player.color })?;
        } else {
            player.ready = true;
        }

        Ok(())
    }

    pub async fn handle_play(
        &mut self,
        color: PlayerColor,
        position: Position,
    ) -> Result<Option<GameEnd>, Error>
    {         
        let (player, opponent_player) = match color {
            PlayerColor::Black => (&self.black, &self.white),
            PlayerColor::White => (&self.white, &self.black),
        };

        if !player.ready {
            player.interface
                .notify_error("player has already declared to be ready")
                .await
                .map_err(|error| Error::NotifyError { error, color: player.color })?;
        } else {
            match self.game.register_player_move(player.color, position).await {
                Ok(res) => {
                    if let Some(end) = res {
                        match end {
                            GameEnd::Draw => {
                                player.interface.notify_result(RelativeGameEnd::Draw)
                                    .await
                                    .map_err(|error| Error::NotifyError { error, color: player.color })?;

                                opponent_player.interface.notify_result(RelativeGameEnd::Draw)
                                    .await
                                    .map_err(|error| Error::NotifyError { error, color: opponent_player.color })?;
                            },
                            GameEnd::Win(winner) => {
                                if winner == player.color {
                                    player.interface.notify_result(RelativeGameEnd::Win)
                                        .await
                                        .map_err(|error| Error::NotifyError { error, color: player.color })?;

                                    opponent_player.interface.notify_result(RelativeGameEnd::Loose)
                                        .await
                                        .map_err(|error| Error::NotifyError { error, color: opponent_player.color })?;
                                } else {
                                    player.interface.notify_result(RelativeGameEnd::Loose)
                                        .await
                                        .map_err(|error| Error::NotifyError { error, color: player.color })?;

                                    opponent_player.interface.notify_result(RelativeGameEnd::Win)
                                        .await
                                        .map_err(|error| Error::NotifyError { error, color: opponent_player.color })?;
                                }
                            }
                        }

                        return Ok(Some(end));
                    } else {
                        opponent_player.interface.notify_turn(position)
                            .await
                            .map_err(|error| Error::NotifyError { error, color: opponent_player.color })?;
                    }
                },
                Err(e) => {
                    player.interface
                        .notify_error(&e.to_string()) // There is surely a proper way to handle this
                        .await
                        .map_err(|error| Error::NotifyError { error, color: player.color })?;

                    return Err(e.into());
                },
            }
        }

        Ok(None)
    }

    pub async fn handle_metadata(
        &mut self,
        color: PlayerColor,
        metadata: PlayerMetadata,
    ) -> Result<(), Error>
    {
        let player = match color {
            PlayerColor::Black => &mut self.black,
            PlayerColor::White => &mut self.white,
        };
    
        player.metadata = Some(metadata);
    
        Ok(())
    }

    pub async fn handle_unknown(
        &self,
        color: PlayerColor,
        content: String,
    )-> Result<(), Error>
    {
        tracing::error!("{} send unknown error: \"{}\"", color, content);

        Ok(())
    }
    
    pub async fn handle_error(
        &self,
        color: PlayerColor,
        content: String,
    ) -> Result<(), Error> {    
        tracing::error!("{} send error: \"{}\"", color, content);

        Ok(())
    }

    pub async fn handle_message(
        &self,
        color: PlayerColor,
        content: String,
    ) -> Result<(), Error> {
        tracing::info!("{} send message: \"{}\"", color, content);
  
        Ok(())
    }

    pub async fn handle_debug(
        &self,
        color: PlayerColor,
        content: String,
    ) -> Result<(), Error> {    
        tracing::debug!("{} send debug: \"{}\"", color, content);

        Ok(())
    }

    pub async fn handle_suggestion(
        &self,
        color: PlayerColor,
        position: Position,
    ) -> Result<(), Error> {
        tracing::info!("{} send suggestion: \"{}\"", color, position);

        Ok(())
    }
}
