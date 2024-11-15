use std::fmt;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum CellStatus
{
    Available,
    Black,
    White,
}


pub enum CheckRowAxis
{
    Horizontal,
    Vertical,
    DiagonalUp,
    DiagonalDown,
}

impl CheckRowAxis
{
    pub const fn value(&self) -> (i8, i8)
    {
        match *self {
            CheckRowAxis::Horizontal => (1, 0),
            CheckRowAxis::Vertical => (0, 1),
            CheckRowAxis::DiagonalUp => (1, -1),
            CheckRowAxis::DiagonalDown => (1, 1),
        }
    }
}

#[derive(Debug, Clone)]
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
    pub fn new(
        size: u8,
    ) -> Self
    {
        Self {
            size: size,
            cells: (0..size)
                .map(|_| vec![CellStatus::Available; size as usize])
                .collect::<Vec<Vec<CellStatus>>>(),
        }
    }

    pub fn set_cell(
        &mut self,
        position: Position,
        new_status: CellStatus,
    ) -> Result<(), SetCellError>
    {
        if position.x > self.size || position.y > self.size {
            return Err(SetCellError::OutOfBounds{position, size: self.size});
        } else if self.cells[position.x as usize][position.y as usize] != CellStatus::Available {
            return Err(SetCellError::UnavailableCell(position));
        }

        self.cells[position.x as usize][position.y as usize] = new_status;
        
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Black,
    White,
}

impl Player
{
    #[allow(dead_code)]
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
