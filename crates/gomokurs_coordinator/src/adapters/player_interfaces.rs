//! This module contains the different adapters of the player interface
//! (implementing both PlayerListener and PlayerNotifier).

#[cfg(feature = "local")]
pub mod local;
#[cfg(feature = "tcp")]
pub mod tcp;
