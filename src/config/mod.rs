//! Configuration management for Obsidium
//!
//! This module handles server configuration including network settings,
//! game rules, and performance tuning options.

pub mod properties;
pub mod server;

pub use properties::ServerProperties;
pub use server::ServerConfig;
