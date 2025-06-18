//! Server configuration settings
//!
//! This module defines the main server configuration structure and
//! provides sensible defaults for all server settings.

use std::net::SocketAddr;
use std::time::Duration;

/// Main server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Server bind address
    pub bind_address: SocketAddr,

    /// Maximum number of concurrent players
    pub max_players: u32,

    /// Server description (MOTD)
    pub motd: String,

    /// Online mode (authentication with Mojang)
    pub online_mode: bool,

    /// Compression threshold in bytes
    pub compression_threshold: Option<u32>,

    /// Connection timeout
    pub connection_timeout: Duration,

    /// View distance in chunks
    pub view_distance: u8,
    /// Simulation distance in chunks  
    pub simulation_distance: u8,

    /// Server favicon (path to 64x64 PNG file or base64 data URL)
    pub favicon: Option<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_address: "0.0.0.0:25565".parse().unwrap(),
            max_players: 20,
            motd: "Welcome to Obsidium - an experimental Minecraft server written in Rust!"
                .to_string(),
            online_mode: true,
            compression_threshold: Some(256),
            connection_timeout: Duration::from_secs(30),
            view_distance: 12,
            simulation_distance: 12,
            favicon: None,
        }
    }
}

impl ServerConfig {
    /// Create a new server configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the server bind address
    pub fn with_bind_address(mut self, addr: SocketAddr) -> Self {
        self.bind_address = addr;
        self
    }

    /// Set the maximum number of players
    pub fn with_max_players(mut self, max: u32) -> Self {
        self.max_players = max;
        self
    }

    /// Set the server MOTD
    pub fn with_motd(mut self, motd: String) -> Self {
        self.motd = motd;
        self
    }

    /// Set online mode
    pub fn with_online_mode(mut self, online: bool) -> Self {
        self.online_mode = online;
        self
    }
    /// Set compression threshold
    pub fn with_compression_threshold(mut self, threshold: Option<u32>) -> Self {
        self.compression_threshold = threshold;
        self
    }

    /// Set server favicon (path to PNG file or base64 data URL)
    pub fn with_favicon(mut self, favicon: Option<String>) -> Self {
        self.favicon = favicon;
        self
    }
}
