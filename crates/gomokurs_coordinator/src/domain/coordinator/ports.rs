//! Define the ports required by the player interfaces manager service.

pub mod service;
pub mod player_interface;

pub use service::{CoordinatorService, GameEngineService};
pub use player_interface::PlayerInterface;
