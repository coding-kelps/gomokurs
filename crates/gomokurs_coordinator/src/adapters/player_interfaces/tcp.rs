//! An implementation of the player interface leveraging TCP.

pub mod tcp;
mod listener;
mod notifier;

pub use tcp::Tcp;