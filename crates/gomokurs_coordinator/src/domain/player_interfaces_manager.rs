//! Facilitates communication of player actions to the manager.

pub mod models;
pub mod ports;
pub mod service;

pub use ports::PlayerInterfacesManagerService;
pub use service::Service as PlayerInterfacesManager;
