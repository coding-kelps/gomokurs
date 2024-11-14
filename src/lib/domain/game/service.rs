use crate::domain::game::models::Turn;
use crate::domain::game::ports::{GameStateRepository, PlayerNotifier};

#[derive(Debug, Clone)]
pub struct Service
where
    R: GameStateRepository,
    N: PlayerNotifier,
{
    repo: R,
    notifier: PlayerNotifier,
}

impl<R> Service<R>
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

impl<R, N> Service<R, N>
where
    R: GameStateRepository,
    N: PlayerNotifier,
{
    pub fn start(&self, req: &StartRequest) -> Result<(), ()> {
        let size = match req.size {
            Some(size) => size,
            None => 10,
        };

        self.repo.init_board(size);

        self.notifier.notify_begin();

        Ok(())
    }

    fn check_row(
        &self,
        board: Board,
        origin: Position,
        axis: CheckRowAxis,
        status: CellStatus,
    ) -> bool
    {
        let mut nb_consecutive = 0u8;

        for i in -5..5 {
            let axis_vec = axis.value();
            let pos = Position {
                x: (origin.x as i32 + (axis_vec.x * i) as i32) as u8,
                y: (origin.y as i32 + (axis_vec.y * i) as i32) as u8,
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
        board: Board,
        last_move: Position,
        player: Player,
    ) -> bool
    {
        self.check_row(board,
                last_move, 
                CheckRowAxis::Horizontal,
                player.into())
            || self.check_row(board, 
                last_move,
                CheckRowAxis::Vertical,
                player.into())
            || self.check_row(board, 
                last_move,
                CheckRowAxis::DiagonalUp,
                player.into(),
            )
            || self.check_row(board, 
                last_move,
                CheckRowAxis::DiagonalDown,
                player.into(),
            )
    }

    pub fn play_turn(&self, req: &PlayTurnRequest) -> Result<(), ()> {
        let mut board = self.repo.get_board();

        if board.set_cell(req.position, CellStatus::Black).is_err() {
            Err(())
        }

        self.repo.register_turn(position, CellStatus::Black);

        if self.check_win(req.position, Player::Black) {
            self.notifier.notify_end();
        }

        Ok(())
    }

    pub fn reset(&self, req: &ResetRequest) -> Result<(), ()> {
        let size = match req.size {
            Some(size) => size,
            None => 10,
        };

        self.repo.reset_board(size);

        Ok(())
    }
}
