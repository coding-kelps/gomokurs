//! [![github]](https://github.com/coding-kelps/gomokurs)
//! 
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! 
//! <br>
//! 
//! A fast and cross-platform gomoku game manager.
//! 
//! <br>
//! 
//! # Details
//!
//! Gomokurs is a Rust implementation of
//! [`Petr Laštovička's Piskvork`](https://github.com/plastovicka/Piskvork), a 
//! gomoku game manager designed to facilitate the training of AI for gomoku. It
//! is designed to support a broader range of platforms and communication
//! protocols, enabling training and testing on distributed systems, while
//! remaining fully compatible with [`Gomocup`](https://gomocup.org/).
//!
//! ## About the Gomocup
//! 
//! The [`Gomocup`](https://gomocup.org/) is an international competition in
//! gomoku AI programming held annually. The competition requires the
//! implemented AI to use the
//! [`Gomocup Protocol`](https://plastovicka.github.io/protocl2en.htm), which
//! relies on standard I/O and is managed by this game manager. For more 
//! details, see the
//! [`Gomocup's gomoku rules`](https://gomocup.org/detail-information/).

pub mod domain;
pub mod adapters;
