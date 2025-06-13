//! Network layer for Obsidium
//!
//! This module handles low-level networking including connection management,
//! packet framing, and the server listener.

pub mod codec;
pub mod connection;
pub mod listener;

pub use connection::Connection;
pub use listener::ServerListener;
