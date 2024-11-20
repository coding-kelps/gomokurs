use crate::domain::game::models::{Position, CellStatus, Board, Player, PlayTurnRequest, PlayTurnError, CheckRowAxis};
use crate::domain::game::ports::{GameService, PlayerClient};
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct Service<C>
where
    C: PlayerClient,
{
    white_player: C,
    _black_player: C,
    board: Board,
}

impl<C> Service<C>
where
    C: PlayerClient,
{
    pub fn new(white_player: C, black_player: C, size: u8) -> Self {
        Self {
            white_player,
            _black_player: black_player,
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
    fn play_turn(&mut self, req: &PlayTurnRequest) -> Result<(), PlayTurnError> {
        if let Err(e) = self.board.set_cell(req.position, CellStatus::Black) {
            return Err(PlayTurnError::Unknown(anyhow!(e)));
        }

        if self.check_win(req.position, Player::Black) {
            self.white_player.request_end()
                .map_err(|e| PlayTurnError::Unknown(anyhow!(e)))?;
        }

        Ok(())
    }
}
