// Todos
//
// X Structure board
// X Une fonction pour placer un pion
// X Une fonction pour print le board
// X System de tour par tour (Faire une structure game)
// - Une fonction pour checker la win
// - Faire une CLI
// - Faire une commande pour jouer depuis la CLI
// X Restructurer en module

mod board;
mod game;
use game::*;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    println!("Welcome gomokurs!");
    let mut g = Game::new(10);
    g.play()?;
    Ok(())
}
