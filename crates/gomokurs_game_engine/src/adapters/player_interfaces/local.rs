//! An implementation of the player interface for a local program.

pub mod local;
mod parsers;
mod listener;
mod notifier;

pub use local::Local;
