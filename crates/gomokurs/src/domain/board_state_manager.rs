//! Manages game state changes and checks for player victories.

pub mod models;
pub mod ports;
pub mod service;

pub use ports::BoardStateManagerService;
pub use service::Service as BoardStateManager;
