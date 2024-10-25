// Todo
//
// - Structure board
// - Une fonction pour checker la win
// - Une fonction pour placer un pion
// - System de tour par tour
// -
use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub enum CellStatus
{
    Available,
    Black,
    White,
}
#[derive(Debug)]
pub struct Board
{
    #[allow(dead_code)]
    size: u8,
    cells: Vec<Vec<CellStatus>>,
}

impl Board
{
    fn new(size: u8) -> Self
    {
        let cells = (0..size)
            .map(|_| vec![CellStatus::Available; size as usize])
            .collect::<Vec<Vec<CellStatus>>>();

        Board { size, cells }
    }

    fn _set_cell(
        &mut self,
        x: u8,
        y: u8,
        new_status: CellStatus,
    ) -> Result<(), Box<dyn Error>>
    {
        if x > self.size || y > self.size {
            return Err(format!("error {} or {} > {}", x, y, self.size).into());
        }
        self.cells[x as usize][y as usize] = new_status;
        Ok(())
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
            .map(|row| {
                row.iter()
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

fn main()
{
    println!("Welcome gomokurs!");
    let b = Board::new(10);
    println!("{b}");
}
