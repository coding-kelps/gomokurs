//! Define the GameManagerService port.

use crate::domain::board_state_manager::models::{Position, GameEnd};
use crate::domain::game_manager::models::{Error, PlayerColor, PlayerDescription};

/// Handles the actions taken by the players, notifying them when necessary,
/// managing their respective timers, and updating the game state when required.
pub trait GameManagerService
{
    /// Initialize the game for both players.
    fn init_game(
        &self,
    ) -> impl std::future::Future<Output = Result<(), Error>>;

    /// Start each player timers, returns a GameEnd if one of the timers ran
    /// out.
    fn run_timers(
        &self,
    ) -> impl std::future::Future<Output = Result<GameEnd, Error>>;

    /// Handle a player "OK" response.
    /// 
    /// An "OK" action is meant to be sent by a player to the manager to signal
    /// he's ready to start the game as a response to the manager's "START"
    /// action. Upon an "OK" action the game manager register the player
    /// readiness.
    /// 
    /// # Arguments
    /// 
    /// * `color` - The color of the player which sent the "OK" response.
    fn handle_ok(
        &mut self,
        color: PlayerColor,
    ) -> impl std::future::Future<Output = Result<(), Error>>;

    /// Handle a player move.
    /// 
    /// The game manager register the player move to the board, pause the player
    /// timer. Then notify and resumes the other player timer, waiting for
    /// his response.
    /// 
    /// # Arguments
    /// 
    /// * `color` - The color of the player which sent the move.
    /// * `position` - The position at which the player wants to play his move.
    fn handle_play(
        &mut self,
        color: PlayerColor,
        position: Position,
    ) -> impl std::future::Future<Output = Result<Option<GameEnd>, Error>>;

    /// Handle a player description.
    /// 
    /// The game manager register the player metadata.
    /// 
    /// # Arguments
    /// 
    /// * `color` - The color of the player which sent the description.
    /// * `description` - The player description sent as a collection of key
    /// value pairs.
    fn handle_description(
        &mut self,
        color: PlayerColor,
        description: PlayerDescription,
    ) -> impl std::future::Future<Output = Result<(), Error>>;

    /// Handle a player unknown error message.
    /// 
    /// A player send this type of action when it didn't recognize a manager
    /// action sent to him. The game manager is expected to log those
    /// messages and end the game.
    /// 
    /// # Arguments
    /// 
    /// * `color` - The color of the player which sent the message.
    /// * `content` - The content of the error message.
    fn handle_unknown(
        &self,
        color: PlayerColor,
        content: String,
    ) -> impl std::future::Future<Output = Result<(), Error>>;

    /// Handle a player error message.
    /// 
    /// A player send this type of action when it did recognize a manager
    /// action sent to him but didn't expected it or its arguments.
    /// The game manager is expected to log those messages and end the game.
    /// 
    /// # Arguments
    /// 
    /// * `color` - The color of the player which sent the message.
    /// * `content` - The content of the error message.
    fn handle_error(
        &self,
        color: PlayerColor,
        content: String,
    ) -> impl std::future::Future<Output = Result<(), Error>>;

    /// Handle a player message.
    /// 
    /// # Arguments
    /// 
    /// * `color` - The color of the player which sent the message.
    /// * `content` - The content of the message.
    fn handle_message(
        &self,
        color: PlayerColor,
        content: String,
    ) -> impl std::future::Future<Output = Result<(), Error>>;

    /// Handle a player debug message.
    /// 
    /// Those kind of messages are only expected in development. The game
    /// manager is only expected to log those messages.
    /// 
    /// # Arguments
    /// 
    /// * `color` - The color of the player which sent the message.
    /// * `content` - The content of the debug message.
    fn handle_debug(
        &self,
        color: PlayerColor,
        content: String,
    ) -> impl std::future::Future<Output = Result<(), Error>>;

    /// Handle a player suggestion.
    /// 
    /// # Arguments
    /// 
    /// * `color` - The color of the player which sent the move.
    /// * `position` - The position at which the player suggests to play his
    /// move.
    fn handle_suggestion(
        &self,
        color: PlayerColor,
        position: Position,
    ) -> impl std::future::Future<Output = Result<(), Error>>;
}
