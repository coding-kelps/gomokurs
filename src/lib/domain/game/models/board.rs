use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CellStatus
{
    Available,
    Black,
    White,
}

#[derive(Debug)]
pub struct Board
{
    pub size: u8,
    pub cells: Vec<Vec<CellStatus>>,
}

#[derive(Debug, Error)]
pub enum SetCellError {
    #[error("index `{0}` points to unavailable cell")]
    UnavailableCell(Position),

    #[error("index `{position}` out of bounds: `{size}`")]
    OutOfBounds {
        position: Position,
        size: u8,
    },
}

impl Board
{
    pub fn set_cell(
        &mut self,
        position: Position,
        new_status: CellStatus,
    ) -> Result<(), SetCellError>
    {
        if position.x > self.size || position.y > self.size {
            Err(SetCellError::OutOfBounds{position, size: self.size})
        } else if self.cells[position.x as usize][position.y as usize] != CellStatus::Available {
            Err(SetCellError::UnavailableCell(position))
        }

        self.cells[position.x as usize][position.y as usize] = new_status;
        
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Player {
    Black,
    White,
}

impl Player
{
    fn switch(&self) -> Player
    {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}

impl Into<CellStatus> for Player
{
    fn into(self) -> CellStatus
    {
        match self {
            Player::Black => CellStatus::Black,
            Player::White => CellStatus::White,
        }
    }
}
