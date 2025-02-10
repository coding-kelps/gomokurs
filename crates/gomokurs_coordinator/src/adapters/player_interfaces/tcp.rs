//! An implementation of the player interface leveraging TCP.

pub mod interface;
mod protocol;
mod handlers;

pub use interface::{CreateTcpPlayerInterfaceConfiguration, TcpPlayerInterface, CreateTcpPlayerInterfaceError};
