use crate::domain::game::models::{Position, CellStatus, Board, Player, PlayTurnRequest, PlayTurnError};
use crate::domain::game::ports::{GameService, PlayerNotifier};
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct Service<N>
where
    N: PlayerNotifier,
{
    notifier: N,
    board: Board,
}

impl<N> Service<N>
where
    N: PlayerNotifier,
{
    pub fn new(notifier: N, size: u8) -> Self {
        Self {
            notifier,
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

enum CheckRowAxis
{
    Horizontal,
    Vertical,
    DiagonalUp,
    DiagonalDown,
}

impl CheckRowAxis
{
    const fn value(&self) -> (i8, i8)
    {
        match *self {
            CheckRowAxis::Horizontal => (1, 0),
            CheckRowAxis::Vertical => (0, 1),
            CheckRowAxis::DiagonalUp => (1, -1),
            CheckRowAxis::DiagonalDown => (1, 1),
        }
    }
}

impl<N> GameService for Service<N>
where
    N: PlayerNotifier,
{
    fn play_turn(&mut self, req: &PlayTurnRequest) -> Result<(), PlayTurnError> {
        if let Err(e) = self.board.set_cell(req.position, CellStatus::Black) {
            return Err(PlayTurnError::Unknown(anyhow!(e)));
        }

        if self.check_win(req.position, Player::Black) {
            self.notifier.notify_end()
                .map_err(|e| PlayTurnError::Unknown(anyhow!(e)))?;
        }

        Ok(())
    }
}
