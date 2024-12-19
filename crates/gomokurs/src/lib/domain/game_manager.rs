//! Player actions and timers handling.
//! 
//! This module defines the Game Manager service, including the
//! [`ports::GameManagerService`] trait and the [`service::Service`]
//! implementation.
//! 
//! The Game Manager is responsible for handling actions taken by players, 
//! notifying them when necessary, and managing their respective timers.


pub mod models;
pub mod ports;
pub mod service;

pub use ports::GameManagerService;
pub use service::Service as GameManager;
