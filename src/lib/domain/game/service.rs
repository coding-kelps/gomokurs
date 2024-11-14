use crate::domain::game::models::{Position, CellStatus, Board, Player, PlayTurnRequest, PlayTurnError};
use crate::domain::game::ports::{GameService, GameStateRepository, PlayerNotifier};
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct Service<R, N>
where
    R: GameStateRepository,
    N: PlayerNotifier,
{
    repo: R,
    notifier: N,
}

impl<R, N> Service<R, N>
where
    R: GameStateRepository,
    N: PlayerNotifier,
{
    pub fn new(repo: R, notifier: N) -> Self {
        Self {
            repo,
            notifier,
        }
    }

    fn check_row(
        &self,
        board: &Board,
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

            if pos.x >= board.size || pos.y >= board.size
            {
                continue;
            } else {
                if board.cells[pos.x as usize][pos.y as usize] == status {
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
        board: &Board,
        last_move: Position,
        player: Player,
    ) -> bool
    {
        self.check_row(&board,
                last_move, 
                CheckRowAxis::Horizontal,
                player.into())
            || self.check_row(&board, 
                last_move,
                CheckRowAxis::Vertical,
                player.into())
            || self.check_row(&board, 
                last_move,
                CheckRowAxis::DiagonalUp,
                player.into(),
            )
            || self.check_row(&board, 
                last_move,
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

impl<R, N> GameService for Service<R, N>
where
    R: GameStateRepository,
    N: PlayerNotifier,
{
    fn play_turn(&mut self, req: &PlayTurnRequest) -> Result<(), PlayTurnError> {
        let mut board = self.repo.get_board()
            .map_err(|e| PlayTurnError::Unknown(anyhow!(e)))?;

        if let Err(e) = board.set_cell(req.position, CellStatus::Black) {
            return Err(PlayTurnError::Unknown(anyhow!(e)));
        }

        self.repo.register_turn(req.position, CellStatus::Black)
            .map_err(|e| PlayTurnError::Unknown(anyhow!(e)))?;

        if self.check_win(&board, req.position, Player::Black) {
            self.notifier.notify_end()
                .map_err(|e| PlayTurnError::Unknown(anyhow!(e)))?;
        }

        Ok(())
    }
}
