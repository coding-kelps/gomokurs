//! A fast and cross-platform gomoku game manager.
//!
//! Gomokurs is a Rust implementation of [`Petr Laštovička's Piskvork`](https://github.com/plastovicka/Piskvork), 
//! a gomoku game manager designed to facilitate the training of AI for gomoku. 
//! It is designed to support a broader range of platforms and communication protocols, 
//! enabling training and testing on distributed systems, while remaining fully compatible with [`Gomocup`](https://gomocup.org/).
//!
//! # Gomocup
//! 
//! The [`Gomocup`](https://gomocup.org/) is an international competition in gomoku AI programming held annually. 
//! The competition requires the implemented AI to use the [`Gomocup Protocol`](https://plastovicka.github.io/protocl2en.htm), 
//! which relies on standard I/O and is managed by this game manager. 
//! For more details, see the [`Gomocup's gomoku rules`](https://gomocup.org/detail-information/).

pub mod domain;
pub mod adapters;
