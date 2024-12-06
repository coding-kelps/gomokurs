use std::collections::HashMap;
use tokio::task::JoinHandle;
use std::sync::Arc;
use crate::domain::game_manager::models::*;
use crate::domain::game_manager::ports::PlayerNotifier;

#[derive()]
pub struct Service<N>
where
    N: PlayerNotifier
{
    games: HashMap<String, GameHandler<N>>,
}

struct GameHandler<N>
where
    N: PlayerNotifier
{
    game: Arc<Game<N>>,
    handle: JoinHandle<Result<GameEnd, PlayError>>,
}

impl<N> GameHandler<N>
where
    N: PlayerNotifier
{
    fn new(
        game: Game<N>
    ) -> Self {
        let mut arc = Arc::new(game);

        Self {
            game: arc.clone(),
            handle: tokio::task::spawn(println!("thread")),
        }
    }
}


impl<N> Service<N>
where
    N: PlayerNotifier
{
    pub fn new() -> Self {
        Self {
            games: HashMap::new(),
        }
    }

    pub fn new_game(
        &mut self,
        black_player: N,
        white_player: N,
    ) -> Result<(), ()> {
        let mut game = Game::new(black_player, white_player, 20);

        self.games.insert(
            "".to_string(),
            GameHandler::new(game),
        );

        Ok(())
    }
}
