//! Facilitates communication of player actions to the manager.

pub mod models;
pub mod ports;
pub mod service;

pub use service::{CreateCoordinatorConfiguration, Service as Coordinator};
pub use ports::CoordinatorService;
