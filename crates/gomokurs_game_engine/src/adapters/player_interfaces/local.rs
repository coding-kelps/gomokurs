//! An implementation of the player interface for a local program.

pub mod local;
mod parsers;
pub mod listener;
pub mod notifier;

pub use local::Local;
