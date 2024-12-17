use std::fmt;
use std::hash::Hash;
use thiserror::Error;

/// A player color (either black or white).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerColor {
    Black,
    White,
}

impl PlayerColor
{
    /// Returns the opposite color.
    pub fn other(&self) -> PlayerColor
    {
        match self {
            PlayerColor::Black => PlayerColor::White,
            PlayerColor::White => PlayerColor::Black,
        }
    }

    /// Change current value of player color for the opposite one.
    pub fn switch(&mut self)
    {
        *self = match *self {
            PlayerColor::Black => PlayerColor::White,
            PlayerColor::White => PlayerColor::Black,
        }
    }
}

impl Into<CellStatus> for PlayerColor
{
    fn into(self) -> CellStatus
    {
        match self {
            PlayerColor::Black => CellStatus::Black,
            PlayerColor::White => CellStatus::White,
        }
    }
}

impl fmt::Display for PlayerColor
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            PlayerColor::Black =>  write!(f, "black player"),
            PlayerColor::White => write!(f, "white player"),
        }
    }
}

/// A 2D coordinates to describe a position on the board.
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        Self {
            x: x,
            y: y
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

/// An enum describing a Board cell status.
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum CellStatus
{
    /// No stone, the cell is available.
    Available,
    // A black stone has been played on this cell.
    Black,
    // A white stone has been played on this cell.
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

/// The gomoku board.
#[derive(Debug, Clone)]
pub struct Board
{
    /// The size of the board.
    pub size: BoardSize,
    /// The cells of the board of a 2D vector.
    pub cells: Vec<Vec<CellStatus>>,
}

/// An error when the setting of a board cell status failed.
#[derive(Debug, Error)]
pub enum SetCellError {
    /// When the cell wasn't available.
    #[error("index `{0}` points to unavailable cell")]
    UnavailableCell(Position),
    /// When the cell coordinates were out of bounds.
    #[error("index `{position}` out of bounds: `{size}`")]
    OutOfBounds {
        position: Position,
        size: BoardSize,
    },
}

/// The 2D size of a Board.
pub type BoardSize = Position;

impl Board
{
    /// Instantiate a new [`Board`] from a given size.
    pub fn new(
        size: BoardSize,
    ) -> Self
    {
        Self {
            size: size,
            cells: (0..size.x)
                .map(|_| vec![CellStatus::Available; size.y as usize])
                .collect::<Vec<Vec<CellStatus>>>(),
        }
    }

    /// Set a given cell from the board at a status. 
    pub fn set_cell(
        &mut self,
        position: Position,
        new_status: CellStatus,
    ) -> Result<(), SetCellError>
    {
        if position.x > self.size.x || position.y > self.size.y {
            return Err(SetCellError::OutOfBounds{position, size: self.size});
        } else if self.cells[position.x as usize][position.y as usize] != CellStatus::Available {
            return Err(SetCellError::UnavailableCell(position));
        }

        self.cells[position.x as usize][position.y as usize] = new_status;
        
        Ok(())
    }

    /// Check if a row contains five of a given status in a row. 
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

            if pos.x >= self.size.x || pos.y >= self.size.y
            {
                continue;
            } else {
                if self.cells[pos.x as usize][pos.y as usize] == status {
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

    /// Check for a player win in the board from the last move.
    pub fn check_win(
        &self,
        last_move: Position,
        player: CellStatus,
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

impl fmt::Display for Board
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result
    {
        let board_as_string: String = self
            .cells
            .iter()
            .map(|col| {
                col.iter()
                    .map(|cell| {
                        match cell {
                            CellStatus::Available => " ",
                            CellStatus::Black => "X",
                            CellStatus::White => "O",
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("|")
            })
            .collect::<Vec<_>>()
            .join("\n");

        writeln!(f, "{}", board_as_string)
    }
}

/// An gomoku match ending.
#[derive(Debug, Clone, Copy)]
pub enum GameEnd {
    /// A Player won.
    Win(PlayerColor),
    /// Draw, no player won.
    Draw,
}

/// An error returned by the board state manager.
#[derive(Debug, Error)]
pub enum Error {
    /// An error returned when a player tried to player while it was not the
    /// player turn to play.
    #[error("it is not `{0}` turn")]
    NotPlayerTurn(PlayerColor),
    /// An error when a cell status setting failed.
    #[error("set cell error: `{0}`")]
    SetCellError(#[from] SetCellError),
    /// An implementation specific error.
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
