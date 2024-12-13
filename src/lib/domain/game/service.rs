use crate::domain::game::ports::{GameService, PlayerClient};
use crate::domain::game::models::*;
use uuid::Uuid;
use tokio::{sync::mpsc::{Receiver, channel}, task::JoinSet};
use std::sync::Arc;

#[derive(Debug, Clone)]
struct Player<C>
where
    C: PlayerClient
{
    pub ready: bool,
    pub client: Arc<C>,
    pub infos: Option<PlayerDescription>,
}


pub struct Game<C>
where
    C: PlayerClient
{
    pub uuid: Uuid,
    black_player: Player<C>,
    white_player: Player<C>,
    board: Board,
    turn_player: PlayerColor,
    _listeners: JoinSet<Result<(), ListenError>>,
    actions_rx: Receiver<(PlayerColor, PlayerAction)>,
}

impl<C> Game<C>
where
    C: PlayerClient
{
    pub fn new(
        black_client: Arc<C>,
        white_client: Arc<C>,
        size: u8,
    ) -> Self
    {
        let (actions_tx_black, actions_rx) = channel::<(PlayerColor, PlayerAction)>(100);
        let actions_tx_white = actions_tx_black.clone();

        let black_listen_client = black_client.clone();
        let white_listen_client = white_client.clone();

        let mut _listeners = JoinSet::new();
        _listeners.spawn(async move { black_listen_client.listen(actions_tx_black, PlayerColor::Black).await });
        _listeners.spawn(async move { white_listen_client.listen(actions_tx_white, PlayerColor::White).await });

        Self {
            uuid: Uuid::new_v4(),
            black_player: Player{ ready: false, client: black_client, infos: None },
            white_player: Player{ ready: false, client: white_client, infos: None },
            board: Board::new(size),
            turn_player: PlayerColor::Black,
            _listeners,
            actions_rx,
        }
    }

    async fn register_player_readiness(
        &mut self,
        player: PlayerColor,
    ) -> Result<(), Error>
    {
        let current_player = match player {
            PlayerColor::Black => &mut self.black_player,
            PlayerColor::White => &mut self.white_player,
        };

        if !current_player.ready {
            current_player.ready = true;
        } else {
            current_player.client
                .notify_error("player has already declared to be ready")
                .await
                .map_err(|error| Error::NotifyError { error, player })?;
        }

        Ok(())
    }


    async fn register_player_move(
        &mut self,
        position: Position,
        player: PlayerColor,
    ) -> Result<Option<GameEnd>, Error>
    {
        let (current_player, other_player) = match player {
            PlayerColor::Black => (&self.black_player, &self.white_player),
            PlayerColor::White => (&self.white_player, &self.black_player),
        };
            
        if !current_player.ready {
            current_player.client.notify_error("player hasn't declared to be ready")
                .await
                .map_err(|error| Error::NotifyError { error, player })?;
        } else if player != self.turn_player {
            current_player.client.notify_error("it isn't player turn")
                .await
                .map_err(|error| Error::NotifyError { error, player })?;
        } else {
            if let Err(e) = self.board.set_cell(position, player.into()) {
                current_player.client.notify_error(&format!("{}", e))
                    .await
                    .map_err(|error| Error::NotifyError { error, player })?;
            } else {
                if self.board.check_win(position, player.into()) {
                    current_player.client.notify_end().await
                        .map_err(|error| Error::NotifyError { error, player })?;
                    other_player.client.notify_end().await
                        .map_err(|error| Error::NotifyError { error, player })?;
    
                    return Ok(Some(GameEnd::Win(player)));
                } else {
                    other_player.client.notify_turn(position).await
                        .map_err(|error| Error::NotifyError { error, player })?;

                    self.turn_player.switch();
                }
            }
        }

        Ok(None)
    }

    async fn register_player_description(
        &mut self,
        description: PlayerDescription,
        player: PlayerColor,
    ) -> Result<(), Error> {
        match player {
            PlayerColor::Black => self.black_player.infos = Some(description),
            PlayerColor::White => self.white_player.infos = Some(description),
        };

        Ok(())
    }

    #[allow(unused_variables)]
    async fn register_unknown(
        &mut self,
        content: String,
        player: PlayerColor,
    ) -> Result<(), Error> {    
        Ok(())
    }

    #[allow(unused_variables)]
    async fn register_error(
        &mut self,
        content: String,
        player: PlayerColor,
    ) -> Result<(), Error> {    
        Ok(())
    }

    #[allow(unused_variables)]
    async fn register_message(
        &mut self,
        content: String,
        player: PlayerColor,
    ) -> Result<(), Error> {    
        Ok(())
    }

    #[allow(unused_variables)]
    async fn register_debug(
        &mut self,
        content: String,
        player: PlayerColor,
    ) -> Result<(), Error> {    
        Ok(())
    }

    #[allow(unused_variables)]
    async fn register_suggestion(
        &mut self,
        position: Position,
        player: PlayerColor,
    ) -> Result<(), Error> {
        Ok(())
    }
}

impl<C> GameService<C> for Game<C>
where
    C: PlayerClient
{
    async fn play(
        &mut self,
    ) -> Result<GameEnd, Error>
    {
        self.black_player.client.notify_start(self.board.size).await
            .map_err(|error| Error::NotifyError { error, player: PlayerColor::Black })?;
        self.white_player.client.notify_start(self.board.size).await
            .map_err(|error| Error::NotifyError { error, player: PlayerColor::White })?;
        self.black_player.client.notify_begin().await
            .map_err(|error| Error::NotifyError { error, player: PlayerColor::Black })?;


        while let Some((player, action)) = self.actions_rx.recv().await {
            println!("received action: {:?} from player {:?}", action, player);

            let _ = match action {
                PlayerAction::Ok => {
                    self.register_player_readiness(player).await?;
                },
                PlayerAction::Play(p) => {
                    if let Some(end) = self.register_player_move(p, player).await? {
                        return Ok(end);
                    } else {
                        continue;
                    }
                },
                PlayerAction::Description(desc) => {
                    self.register_player_description(desc, player).await?;
                },
                PlayerAction::Unknown(c) => {
                    self.register_unknown(c, player).await?;
                },
                PlayerAction::Error(c) => {
                    self.register_error(c, player).await?;
                },
                PlayerAction::Message(c) => {
                    self.register_message(c, player).await?;
                },
                PlayerAction::Debug(c) => {
                    self.register_debug(c, player).await?;
                },
                PlayerAction::Suggestion(p) => {
                    self.register_suggestion(p, player).await?;
                },
            };
        }

        Err(Error::ActionsChannelAbruptlyClose)
    }
}