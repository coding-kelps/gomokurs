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
    async fn handle_player_action(
        &mut self,
        action: PlayerAction,
        #[allow(unused_variables)]
        player: PlayerColor,
    ) -> Result<Option<GameEnd>, PlayError> {
        match action {
            PlayerAction::Ok => {
                if player == PlayerColor::Black {
                    if !self.black_player.ready {
                        self.black_player.ready = true;
                    } else {
                        let _ = self.black_player.notifier.notify_error("player has already declared to be ready").await;
                    }
                } else {
                    if !self.white_player.ready {
                        self.white_player.ready = true;
                    } else {
                        let _ = self.white_player.notifier.notify_error("player has already declared to be ready").await;
                    }
                }
            },
            PlayerAction::Play(position) => {
                if player == PlayerColor::Black {
                    if !self.black_player.ready {
                        let _ = self.black_player.notifier.notify_error("player isn't ready").await;
                    } else if player != self.turn_player {
                        let _ = self.black_player.notifier.notify_error("it isn't player turn").await;
                    } else {
                        let _ = self.board.set_cell(position, player.into());
    
                        if self.board.check_win(position, player.into()) {    
                            let _ = self.black_player.notifier.notify_end().await;
                            let _ = self.white_player.notifier.notify_end().await;

                            return Ok(Some(GameEnd::Win(player)))
                        }
                    }
                } else {
                    if !self.white_player.ready {
                        let _ = self.white_player.notifier.notify_error("player isn't ready").await;
                    } else if player != self.turn_player {
                        let _ = self.white_player.notifier.notify_error("it isn't player turn").await;
                    } else {
                        let _ = self.board.set_cell(position, player.into());
    
                        if self.board.check_win(position, player.into()) {    
                            let _ = self.white_player.notifier.notify_end().await;
                            let _ = self.black_player.notifier.notify_end().await;

                            return Ok(Some(GameEnd::Win(player)))
                        }
                    }
                }
            },
            PlayerAction::Description(desc) => {
                if player == PlayerColor::Black {
                    self.black_player.infos = Some(desc);
                } else {
                    self.white_player.infos = Some(desc);
                }
            },
            // TO DO
            // Not urgent though the message should
            // only be logged in the output.
            PlayerAction::Unknown(_) => (),
            PlayerAction::Error(_) => (),
            PlayerAction::Message(_) => (),
            PlayerAction::Debug(_) => (),
            PlayerAction::Suggestion(_) => (),
        };

        Ok(None)
    }
}