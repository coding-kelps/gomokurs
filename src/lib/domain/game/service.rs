use crate::domain::game::models::Turn;
use crate::domain::game::ports::GameStateRepository;

#[derive(Debug, Clone)]
pub struct Service
where
    R: GameStateRepository
{
    repo: R
}

impl<R> Service<R>
where
    R: GameStateRepository
{
    pub fn new(repo: R) -> Self {
        Self {
            repo,
        }
    }
}

impl<R> Service<R>
where
    R: GameStateRepository
{
    fn start(&self, req: &StartRequest) -> Result<(), ()> {
        let size = match req.size {
            Some(size) => size,
            None => 10,
        };

        self.repo.init_board(size);

        Ok(())
    }

    fn play_turn(&self, req: &PlayTurnRequest) -> Result<(), ()> {
        let mut board = self.repo.get_board();

        if board.set_cell(req.position, CellStatus::Black).is_err() {
            Err(())
        }

        self.repo.register_turn(position, CellStatus::Black);

        Ok(())
    }

    fn reset(&self, req: &ResetRequest) -> Result<(), ()> {
        let size = match req.size {
            Some(size) => size,
            None => 10,
        };

        self.repo.reset_board(size);

        Ok(())
    }
}
