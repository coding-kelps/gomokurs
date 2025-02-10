use crate::domain::coordinator::ports::PlayerInterface;
use crate::domain::coordinator::models::{PlayerColor, PlayerMetadata};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Player<I>
where
    I: PlayerInterface
{
    /// The player's assigned color (either black or white).
    pub color: PlayerColor,
    /// Indicates if the player has declared readiness to play.
    pub ready: bool,
    /// Metadata about the player as key-value pairs.
    pub metadata: Option<PlayerMetadata>,
    /// The notifier used to communicate with the player program.
    pub interface: Arc<I>,
}

impl<I> Player<I>
where
    I: PlayerInterface
{
    pub fn new(
        color: PlayerColor,
        interface: Arc<I>,
    ) -> Self {
        Self {
            color,
            ready: false,
            metadata: None,
            interface: interface,
        }
    }
}