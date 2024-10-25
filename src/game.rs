use std::{cell::Cell, error::Error, io};

use regex::Regex;

use crate::board::*;

#[derive(Debug)]
enum Player
{
    Black,
    White,
}

impl Player
{
    fn switch(self) -> Player
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

struct Game
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
        while self.over != false {
            let pos = loop {
                let res = self.player_input(&current_player);
                if res.is_ok() {
                    break res.unwrap();
                }
            };
            self.board.set_cell(pos.0, pos.1, current_player.into());
            self.over = self.check_win(pos);
        }
        Ok(current_player)
    }

    fn check_row(
        &self,
        origin: (u16, u16),
        axis: (u16, u16),
        status: CellStatus,
    ) -> bool
    {
        let _corrected_origin = origin;
        let mut nb_consecutive = 0u8;

        (0..10).into_iter();

        false
    }

    fn check_win(
        &self,
        last_move: (u16, u16),
    ) -> bool
    {
        false
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

        match re.captures(&buffer) {
            Some(caps) => {
                // TODO: Add custom error messages
                Ok((caps[0].parse::<u16>()?, caps[1].parse::<u16>()?))
            }
            None => Err("Invalid input format".into()),
        }
    }
}
