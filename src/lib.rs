//! # Bitcask Key-Value Storage Engine in Idiomatic Rust
//!
//! An open-source, high-performance, log-structured hash table key-value storage engine
//! inspired by the original Bitcask design paper (*Justin Sheehy & Marc de Kruijf, Basho Technologies*).

pub mod config;
pub mod error;
pub mod record;
