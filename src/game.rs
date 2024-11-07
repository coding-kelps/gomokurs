use std::{error::Error, io};

use regex::Regex;

use crate::board::*;

#[derive(Debug, Clone, Copy)]
pub enum Player
{
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

pub struct Game
{
    board: Board,
    nb_turn: u16,
    over: bool,
}

impl Game
{
    pub fn new(size: u8) -> Self
    {
        let board = Board::new(size);
        Game {
            board,
            nb_turn: 0,
            over: false,
        }
    }

    pub fn play(&mut self) -> Result<Player, Box<dyn Error>>
    {
        let mut current_player = Player::White;
        while self.over != true {
            println!("{}", self.board);
            let pos = loop {
                let res = self.player_input(&current_player);
                match res {
                    Ok(input) => break input,
                    Err(e) => eprintln!("{e}"),
                }
            };
            self.board.set_cell(pos.0, pos.1, current_player.into())?;
            self.over = self.check_win(pos, current_player);
            current_player = current_player.switch();
            self.nb_turn += 1;
        }
        Ok(current_player)
    }

    fn check_row(
        &self,
        origin: (u16, u16),
        axis: CheckRowAxis,
        status: CellStatus,
    ) -> bool
    {
        let mut nb_consecutive = 0u8;

        for i in -5..5 {
            let axis_vec = axis.value();
            let pos: (i32, i32) = (
                origin.0 as i32 * (axis_vec.0 * i) as i32,
                origin.1 as i32 * (axis_vec.1 * i) as i32,
            );

            if pos.0 < 0
                || pos.1 < 0
                || pos.0 >= self.board.size as i32
                || pos.1 >= self.board.size as i32
            {
                continue;
            } else {
                if self.board.cells[pos.0 as usize][pos.1 as usize] == status {
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
        last_move: (u16, u16),
        player: Player,
    ) -> bool
    {
        self.check_row(last_move, CheckRowAxis::Horizontal, player.into())
            || self.check_row(last_move, CheckRowAxis::Vertical, player.into())
            || self.check_row(
                last_move,
                CheckRowAxis::DiagonalUp,
                player.into(),
            )
            || self.check_row(
                last_move,
                CheckRowAxis::DiagonalDown,
                player.into(),
            )
    }

    fn player_input(
        &self,
        player: &Player,
    ) -> Result<(u16, u16), Box<dyn Error>>
    {
        let mut buffer = String::new();

        println!(
            "{:?}'s turn, please type your play coordinate (X, Y): ",
            player
        );
        io::stdin().read_line(&mut buffer)?;

        let re = Regex::new(r"^(\d+), (\d+)$")?;

        match re.captures(&buffer.trim()) {
            Some(caps) => {
                // TODO: Add custom error messages
                let x = caps[1].parse::<u16>()?;
                let y = caps[2].parse::<u16>()?;
                Ok((x, y))
            }
            None => Err("Invalid input format".into()),
        }
    }
}
