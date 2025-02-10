use crate::domain::game_engine::models::*;
use crate::domain::game_engine::ports::GameEngineService;
use std::sync::Arc;
use tokio::time::Duration;

#[derive(Debug, Clone)]
pub struct Service
{
    board: Board,
    turn_player: PlayerColor,
    black_player_timer: Arc<Timer>,
    white_player_timer: Arc<Timer>,
}

impl Service
{
    pub fn new(
        size: BoardSize,
        turn_duration: Duration,
        match_duration: Duration,
    ) -> Self {
        Self {
            board: Board::new(size),
            turn_player: PlayerColor::Black,
            black_player_timer: Arc::new(Timer::new(turn_duration, match_duration)),
            white_player_timer: Arc::new(Timer::new(turn_duration, match_duration)),
        }
    }
}

impl GameEngineService for Service
{
    async fn get_board_size(
        &self,
    ) -> BoardSize {
        return self.board.size;
    }

    async fn run_timers(
        &self,
    ) -> Result<GameEnd, Error>
    {
        tokio::select! {
            _ = self.black_player_timer.run(false) => {
                Ok(GameEnd::Win(PlayerColor::White))
            },
            _ = self.white_player_timer.run(true) => {
                Ok(GameEnd::Win(PlayerColor::Black))
            },
        }
    }

    async fn register_move(
        &mut self,
        color: PlayerColor,
        position: Position,
    ) -> Result<Option<GameEnd>, Error>
    {
        if color != self.turn_player {
            Err(Error::NotPlayerTurn(color))
        } else {
            self.board.set_cell(position, color.into())?;

            if self.board.check_win(position).await {
                Ok(Some(GameEnd::Win(color)))
            } else {
                self.turn_player.switch();

                Ok(None)
            }
        }
    }

    async fn reset(
        &mut self,
    ) -> Result<(), Error> {
        self.board = Board::new(self.board.size);

        Ok(())
    }
}
