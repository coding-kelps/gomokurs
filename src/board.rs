use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq)]
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

impl Board
{
    pub fn new(size: u8) -> Self
    {
        let cells = (0..size)
            .map(|_| vec![CellStatus::Available; size as usize])
            .collect::<Vec<Vec<CellStatus>>>();

        Board { size, cells }
    }

    pub fn set_cell(
        &mut self,
        x: u16,
        y: u16,
        new_status: CellStatus,
    ) -> Result<(), Box<dyn Error>>
    {
        if x as u8 > self.size || y as u8 > self.size {
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
