use crate::domain::gomoku::ports::GomokuService;
use crate::domain::gomoku::models::{Board, PlayerColor, Position, GameEnd, Error};

pub struct Service
{
    board: Board,
    turn_player: PlayerColor,
}

impl Service
{
    pub fn new(
        size: u8,
    ) -> Self
    {
        Self {
            board: Board::new(size),
            turn_player: PlayerColor::Black,
        }
    }
}

impl GomokuService for Service
{
    async fn play_move(
        &mut self,
        color: PlayerColor,
        position: Position,
    ) -> Result<Option<GameEnd>, Error>
    {            
        if color != self.turn_player {
            Err(Error::NotPlayerTurn(color))
        } else {
            self.board.set_cell(position, color.into())?;

            if self.board.check_win(position, color.into()) {
                Ok(Some(GameEnd::Win(color)))
            } else {
                self.turn_player.switch();

                Ok(None)
            }
        }
    }
}