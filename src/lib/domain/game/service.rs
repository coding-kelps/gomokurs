use crate::domain::game::ports::{GameService, PlayerClient};
use crate::domain::game::models::*;
use uuid::Uuid;
use tokio::{sync::mpsc::channel, task::JoinSet};
use std::sync::Arc;

#[derive(Debug, Clone)]
struct Player<C>
where
    C: PlayerClient
{
    pub ready: bool,
    pub client: Arc<C>,
    pub infos: Option<PlayerInformations>,
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
}

impl<C> Game<C>
where
    C: PlayerClient
{
    pub fn new(
        black_client: C,
        white_client: C,
        size: u8,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            black_player: Player{ ready: false, client: Arc::new(black_client), infos: None },
            white_player: Player{ ready: false, client: Arc::new(white_client), infos: None },
            board: Board::new(size),
            turn_player: PlayerColor::Black,
        }
    }

        async fn register_ok(
            &mut self,
            player: PlayerColor,
        ) -> Result<(), ()> {
        match player {
            PlayerColor::Black => {
                if !self.black_player.ready {
                    self.black_player.ready = true;
                } else {
                    let _ = self.black_player.client.notify_error("player has already declared to be ready").await;
                }
            },
            PlayerColor::White => {
                if !self.white_player.ready {
                    self.white_player.ready = true;
                } else {
                    let _ = self.white_player.client.notify_error("player has already declared to be ready").await;
                }
            },
        };

        Ok(())
    }


    async fn register_move(
        &mut self,
        position: Position,
        player: PlayerColor,
    ) -> Result<(), ()>
    {
        let (current_player, other_player) = match player {
            PlayerColor::Black => (&self.black_player, &self.white_player),
            PlayerColor::White => (&self.white_player, &self.black_player),
        };
            
        if !current_player.ready {
            let _ = current_player.client.notify_error("player hasn't declared to be ready").await;

            println!("not ready");
        } else if player != self.turn_player {
            let _ = current_player.client.notify_error("it isn't player turn").await;

            println!("not turn");
        } else {
            let _ = self.board.set_cell(position, player.into());

            if self.board.check_win(position, player.into()) {
                let _ = current_player.client.notify_end().await;
                let _ = other_player.client.notify_end().await;

                println!("player {:?} won!", player);
            } else {
                let _ = other_player.client.notify_turn(position).await;

                self.turn_player.switch();
            }
        }

        Ok(())
    }

    async fn register_description(
        &mut self,
        description: PlayerInformations,
        player: PlayerColor,
    ) -> Result<(), ()> {
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
    ) -> Result<(), ()> {    
        Ok(())
    }

    #[allow(unused_variables)]
    async fn register_error(
        &mut self,
        content: String,
        player: PlayerColor,
    ) -> Result<(), ()> {    
        Ok(())
    }

    #[allow(unused_variables)]
    async fn register_message(
        &mut self,
        content: String,
        player: PlayerColor,
    ) -> Result<(), ()> {    
        Ok(())
    }

    #[allow(unused_variables)]
    async fn register_debug(
        &mut self,
        content: String,
        player: PlayerColor,
    ) -> Result<(), ()> {    
        Ok(())
    }

    #[allow(unused_variables)]
    async fn register_suggestion(
        &mut self,
        position: Position,
        player: PlayerColor,
    ) -> Result<(), ()> {
        Ok(())
    }
}

impl<C> GameService<C> for Game<C>
where
    C: PlayerClient
{
    async fn play(
        &mut self,
    ) -> Result<(), ()>
    {
        let (tx_black, mut rx) = channel::<(PlayerColor, PlayerAction)>(100);
        let tx_white = tx_black.clone();

        let black_client = self.black_player.client.clone();
        let white_client = self.white_player.client.clone();

        let mut set = JoinSet::new();
        set.spawn(async move { black_client.listen(tx_black, PlayerColor::Black).await });
        set.spawn(async move { white_client.listen(tx_white, PlayerColor::White).await });

        let _ = self.black_player.client.notify_start(self.board.size).await;
        let _ = self.white_player.client.notify_start(self.board.size).await;
        let _ = self.black_player.client.notify_begin().await;

        while let Some((player, action)) = rx.recv().await {
            println!("received action: {:?} from player {:?}", action, player);

            let _ = match action {
                PlayerAction::Ok => self.register_ok(player).await,
                PlayerAction::Play(p) => self.register_move(p, player).await,
                PlayerAction::Description(desc) => self.register_description(desc, player).await,
                PlayerAction::Unknown(c) => self.register_unknown(c, player).await,
                PlayerAction::Error(c) => self.register_error(c, player).await,
                PlayerAction::Message(c) => self.register_message(c, player).await,
                PlayerAction::Debug(c) => self.register_debug(c, player).await,
                PlayerAction::Suggestion(p) => self.register_suggestion(p, player).await,
            };
        }

        Ok(())
    }
}