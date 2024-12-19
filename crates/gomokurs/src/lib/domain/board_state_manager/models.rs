//! Models for the board state manager port in a Gomoku game.

use std::fmt;
use std::hash::Hash;
use thiserror::Error;

/// Represents a player's color in the game: either black or white.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerColor {
    Black,
    White,
}

impl PlayerColor
{
    /// Returns the opponent's color.
    pub fn other(&self) -> PlayerColor
    {
        match self {
            PlayerColor::Black => PlayerColor::White,
            PlayerColor::White => PlayerColor::Black,
        }
    }

    /// Toggles the current color to the opposite color.
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

/// Represents a position on the board using 2D coordinates.
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Position {
    /// Creates a new `Position` with the given coordinates.
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

/// Represents the status of a cell on the board.
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum CellStatus
{
    /// The cell is available for a move.
    Available,
    /// A black stone occupies the cell.
    Black,
    /// A white stone occupies the cell.
    White,
}

/// Defines the direction of a row for win checks.
pub enum CheckRowAxis
{
    /// Horizontal row (left-to-right).
    Horizontal,
    /// Vertical row (top-to-bottom).
    Vertical,
    /// Diagonal row (bottom-left to top-right).
    DiagonalUp,
    /// Diagonal row (top-left to bottom-right).
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

/// Represents the game board in a Gomoku match.
#[derive(Debug, Clone)]
pub struct Board
{
    /// Dimensions of the board.
    pub size: BoardSize,
    /// Current state of each cell on the board.
    pub cells: Vec<Vec<CellStatus>>,
}

/// Errors related to board operations.
#[derive(Debug, Error)]
pub enum SetCellError {
    /// The cell is not available for a move.
    #[error("index `{0}` points to unavailable cell")]
    UnavailableCell(Position),
    /// The cell is outside the board boundaries.
    #[error("index `{position}` out of bounds: `{size}`")]
    OutOfBounds {
        position: Position,
        size: BoardSize,
    },
}

/// The size of the board as a 2D dimension.
pub type BoardSize = Position;

impl Board
{
    /// Creates a new empty board of the specified size.
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

    /// Sets the status of a specific cell.
    /// 
    /// # Arguments
    /// 
    /// * `position` - The position of the board's cell to set.
    /// * `new_status` - The new status to assign to the cell.
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

    /// Checks if a row contains at least five consecutive cells of the same status.
    /// 
    /// # Arguments
    /// 
    /// - `origin`: The starting position for the check.
    /// - `axis`: The direction to check.
    async fn check_row(
        &self,
        origin: Position,
        axis: CheckRowAxis,
    ) -> bool
    {
        let status = self.cells[origin.x as usize][origin.y as usize];
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

    /// Checks if a move results in a win.
    /// 
    /// # Arguments
    /// 
    /// - `played_move`: The position of the last move.
    pub async fn check_win(
        &self,
        played_move: Position,
    ) -> bool
    {
        tokio::select! {
            true = self.check_row(
                played_move, CheckRowAxis::Horizontal) => true,
            true = self.check_row(
                played_move, CheckRowAxis::Vertical) => true,
            true = self.check_row(
                played_move, CheckRowAxis::DiagonalUp) => true,
            true = self.check_row(
                played_move, CheckRowAxis::DiagonalDown) => true,
            else => {
                false
            }
        }
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

/// Represents the end state of a Gomoku game.
#[derive(Debug, Clone, Copy)]
pub enum GameEnd {
    /// A player has won the game.
    Win(PlayerColor),
    /// The game ended in a draw.
    Draw,
}

/// An error returned by the board state manager.
#[derive(Debug, Error)]
pub enum Error {
    /// Attempted to play out of turn.
    #[error("it is not `{0}` turn")]
    NotPlayerTurn(PlayerColor),
    /// An error occurred while setting a cell's status.
    #[error("set cell error: `{0}`")]
    SetCellError(#[from] SetCellError),
    /// For implementation-specific error.
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
