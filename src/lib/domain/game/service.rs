use crate::domain::game::models::*;
use crate::domain::game::ports::{GameService, PlayerClient};
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct Service<C>
where
    C: PlayerClient,
{
    black_player: C,
    white_player: C,
    board: Board,
}

impl<C> Service<C>
where
    C: PlayerClient,
{
    pub fn new(black_player: C, white_player: C, size: u8) -> Self {
        Self {
            black_player,
            white_player,
            board: Board::new(size),
        }
    }

    fn check_row(
        &self,
        origin: Position,
        axis: CheckRowAxis,
        status: CellStatus,
    ) -> bool
    {
        let mut nb_consecutive = 0u8;

        for i in -5..5 {
            let axis_vec = axis.value();
            let pos = Position {
                x: (origin.x as i32 + (axis_vec.0 * i) as i32) as u8,
                y: (origin.y as i32 + (axis_vec.1 * i) as i32) as u8,
            };

            if pos.x >= self.board.size || pos.y >= self.board.size
            {
                continue;
            } else {
                if self.board.cells[pos.x as usize][pos.y as usize] == status {
                    nb_consecutive += 1;

                    if nb_consecutive >= 5 {
                        return true;
                    }
                } else {
                    nb_consecutive = 0;
                }
            }
        }

        false
    }

    fn check_win(
        &self,
        last_move: Position,
        player: Player,
    ) -> bool
    {
        self.check_row(last_move, 
                CheckRowAxis::Horizontal,
                player.into())
            || self.check_row(last_move,
                CheckRowAxis::Vertical,
                player.into())
            || self.check_row(last_move,
                CheckRowAxis::DiagonalUp,
                player.into(),
            )
            || self.check_row(last_move,
                CheckRowAxis::DiagonalDown,
                player.into(),
            )
    }
}

impl<C> GameService for Service<C>
where
    C: PlayerClient,
{
    async fn play(&mut self, _req: &PlayRequest) -> Result<GameEnd, PlayError> {
        for client in [& mut self.black_player, & mut self.white_player] {
            client.request_start(self.board.size).await
                .map_err(|e: RequestStartError| PlayError::Unknown(anyhow!(e)))?;
        }

        let mut player = Player::Black;

        let mut last_move = self.black_player.request_begin().await
            .map_err(|e: RequestBeginError| PlayError::Unknown(anyhow!(e)))?;

        self.board.set_cell(last_move, CellStatus::Black)
            .map_err(|e: SetCellError| PlayError::Unknown(anyhow!(e)))?;

        for _ in 1..(self.board.size as u32).pow(2) {
            player = player.switch();
            let client = match player {
                Player::Black => &mut self.black_player,
                Player::White => &mut self.white_player,
            };

            last_move = client.request_turn(last_move).await
                .map_err(|e: RequestTurnError| PlayError::Unknown(anyhow!(e)))?;

            self.board.set_cell(last_move, player.into())
                .map_err(|e: SetCellError| PlayError::Unknown(anyhow!(e)))?;

            if self.check_win(last_move, player) {
                for c in [& mut self.black_player, & mut self.white_player] {
                    c.request_end()
                        .await
                        .map_err(|e: RequestEndError| PlayError::Unknown(anyhow!(e)))?;
                }

                return Ok(GameEnd::Win(player))
            }
        }

        Ok(GameEnd::Draw)
    }
}
