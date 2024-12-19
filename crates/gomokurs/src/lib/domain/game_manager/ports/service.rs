//! Defines the `GameManagerService` port.

use crate::domain::board_state_manager::models::{Position, GameEnd};
use crate::domain::game_manager::models::{Error, PlayerColor, PlayerDescription};

/// A service responsible for managing the game lifecycle, player interactions,
/// timers, and game state updates.
pub trait GameManagerService
{
    /// Initializes the game for both players.
    ///
    /// This method sets up the initial game state and prepares the game manager
    /// for player interactions.
    fn init_game(
        &self,
    ) -> impl std::future::Future<Output = Result<(), Error>>;

    /// Starts the timers for both players and monitors for timeout events.
    ///
    /// Returns a `GameEnd` result if one of the timers runs out.
    fn run_timers(
        &self,
    ) -> impl std::future::Future<Output = Result<GameEnd, Error>>;

    /// Handles a player's "OK" response.
    ///
    /// An "OK" response signals the player's readiness to start the game in
    /// response to the manager's "START" action. This method updates the game
    /// manager to reflect the player's readiness.
    ///
    /// # Arguments
    ///
    /// * `color` - The color representing the player sending the "OK" response.
    fn handle_ok(
        &mut self,
        color: PlayerColor,
    ) -> impl std::future::Future<Output = Result<(), Error>>;

    /// Processes a player's move.
    ///
    /// This method updates the board with the player's move, pauses the current
    /// player's timer, notifies the opponent, and resumes their timer.
    ///
    /// # Arguments
    ///
    /// * `color` - The color representing the player making the move.
    /// * `position` - The position where the player wants to place their piece.
    fn handle_play(
        &mut self,
        color: PlayerColor,
        position: Position,
    ) -> impl std::future::Future<Output = Result<Option<GameEnd>, Error>>;

    /// Registers metadata for a player.
    ///
    /// # Arguments
    ///
    /// * `color` - The color representing the player providing the description.
    /// * `description` - The player's description as key-value metadata pairs.
    fn handle_description(
        &mut self,
        color: PlayerColor,
        description: PlayerDescription,
    ) -> impl std::future::Future<Output = Result<(), Error>>;

    /// Handles an unknown error message from a player.
    ///
    /// This type of message is sent when a player cannot recognize an action sent
    /// by the manager. The game manager should log the message and end the game.
    ///
    /// # Arguments
    ///
    /// * `color` - The color representing the player sending the message.
    /// * `content` - The content of the error message.
    fn handle_unknown(
        &self,
        color: PlayerColor,
        content: String,
    ) -> impl std::future::Future<Output = Result<(), Error>>;

    /// Handles an error message from a player.
    ///
    /// This type of message indicates that the player recognized the manager's
    /// action but found it unexpected or invalid. The game manager should log
    /// the message and end the game.
    ///
    /// # Arguments
    ///
    /// * `color` - The color representing the player sending the message.
    /// * `content` - The content of the error message.
    fn handle_error(
        &self,
        color: PlayerColor,
        content: String,
    ) -> impl std::future::Future<Output = Result<(), Error>>;

    /// Handles a generic message from a player.
    ///
    /// # Arguments
    ///
    /// * `color` - The color representing the player sending the message.
    /// * `content` - The content of the message.
    fn handle_message(
        &self,
        color: PlayerColor,
        content: String,
    ) -> impl std::future::Future<Output = Result<(), Error>>;

    /// Handles a debug message from a player.
    ///
    /// These messages are expected only during development. The game manager is
    /// only required to log the message.
    ///
    /// # Arguments
    ///
    /// * `color` - The color representing the player sending the message.
    /// * `content` - The content of the debug message.
    fn handle_debug(
        &self,
        color: PlayerColor,
        content: String,
    ) -> impl std::future::Future<Output = Result<(), Error>>;

    /// Handles a player's move suggestion.
    ///
    /// # Arguments
    ///
    /// * `color` - The color representing the player providing the suggestion.
    /// * `position` - The position suggested by the player for their move.
    fn handle_suggestion(
        &self,
        color: PlayerColor,
        position: Position,
    ) -> impl std::future::Future<Output = Result<(), Error>>;
}
