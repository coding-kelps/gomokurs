//! Define the ports required by the player interfaces manager service.

pub mod service;
pub mod player_listener;

pub use service::PlayerInterfacesManagerService;
pub use player_listener::PlayerListener;
