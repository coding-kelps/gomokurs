use crate::domain::game::ports::{GameService, PlayerNotifier};
use crate::domain::game::models::*;
use uuid::Uuid;

#[derive(Debug, Clone)]
struct Player<N>
where
    N: PlayerNotifier
{
    pub ready: bool,
    pub notifier: N,
    pub infos: Option<PlayerInformations>,
}


pub struct Service<N>
where
    N: PlayerNotifier
{
    pub uuid: Uuid,
    black_player: Player<N>,
    white_player: Player<N>,
    board: Board,
    turn_player: PlayerColor,
}

impl<N> Service<N>
where
    N: PlayerNotifier
{
    pub fn new(
        black_notifier: N,
        white_notifier: N,
        size: u8,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            black_player: Player{ ready: false, notifier: black_notifier, infos: None },
            white_player: Player{ ready: false, notifier: white_notifier, infos: None },
            board: Board::new(size),
            turn_player: PlayerColor::Black,
        }
    }
}

impl<N> GameService<N> for Service<N>
where
    N: PlayerNotifier
{
    async fn register_ok(
            &mut self,
            player: PlayerColor,
        ) -> Result<(), ()> {
        match player {
            PlayerColor::Black => {
                if !self.black_player.ready {
                    self.black_player.ready = true;
                } else {
                    let _ = self.black_player.notifier.notify_error("player has already declared to be ready").await;
                }
            },
            PlayerColor::White => {
                if !self.white_player.ready {
                    self.white_player.ready = true;
                } else {
                    let _ = self.white_player.notifier.notify_error("player has already declared to be ready").await;
                }
            },
        };

        Ok(())
    }


    async fn register_move(
            &mut self,
            position: Position,
            player: PlayerColor,
        ) -> Result<(), ()> {
        match player {
            PlayerColor::Black => {
                if !self.black_player.ready {
                    let _ = self.black_player.notifier.notify_error("player hasn't declared to be ready").await;
                } else if player != self.turn_player {
                    let _ = self.black_player.notifier.notify_error("it isn't player turn").await;
                } else {
                    let _ = self.board.set_cell(position, player.into());

                    if self.board.check_win(position, player.into()) {
                        let _ = self.black_player.notifier.notify_end().await;
                        let _ = self.white_player.notifier.notify_end().await;
                    }
                }
            },
            PlayerColor::White => {
                if !self.white_player.ready {
                    let _ = self.white_player.notifier.notify_error("player hasn't declared to be ready").await;
                } else if player != self.turn_player {
                    let _ = self.white_player.notifier.notify_error("it isn't player turn").await;
                } else {
                    let _ = self.board.set_cell(position, player.into());

                    if self.board.check_win(position, player.into()) {    
                        let _ = self.white_player.notifier.notify_end().await;
                        let _ = self.black_player.notifier.notify_end().await;
                    }
                }
            },
        };

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