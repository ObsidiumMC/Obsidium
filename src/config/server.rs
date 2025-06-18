//! Server configuration settings
//!
//! This module defines the main server configuration structure and
//! provides sensible defaults for all server settings.

use std::net::SocketAddr;
use std::path::Path;
use std::time::Duration;

use crate::config::properties::ServerProperties;
use crate::error::ServerError;

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

    /// Load configuration from server.properties file
    pub fn from_properties_file<P: AsRef<Path>>(path: P) -> Result<Self, ServerError> {
        let props = ServerProperties::load_from_file(path)?;
        Self::from_properties(props)
    }

    /// Load configuration from server.properties file, using defaults if file doesn't exist
    pub fn from_properties_file_or_default<P: AsRef<Path>>(path: P) -> Result<Self, ServerError> {
        let props = ServerProperties::load_from_file_or_default(path)?;
        Self::from_properties(props)
    }

    /// Create configuration from ServerProperties
    pub fn from_properties(props: ServerProperties) -> Result<Self, ServerError> {
        let server_ip = props.server_ip().unwrap_or(&String::new()).clone();
        let server_port = props.server_port();
        
        let bind_address = if server_ip.is_empty() {
            format!("0.0.0.0:{}", server_port)
        } else {
            format!("{}:{}", server_ip, server_port)
        };

        let bind_address: SocketAddr = bind_address.parse()
            .map_err(|e| ServerError::Protocol(format!("Invalid bind address: {}", e)))?;

        let compression_threshold = match props.network_compression_threshold() {
            -1 => None,
            n if n >= 0 => Some(n as u32),
            _ => Some(256),
        };

        Ok(Self {
            bind_address,
            max_players: props.max_players(),
            motd: props.motd().to_string(),
            online_mode: props.online_mode(),
            compression_threshold,
            connection_timeout: Duration::from_secs(30),
            view_distance: props.view_distance(),
            simulation_distance: props.simulation_distance(),
            favicon: None,
        })
    }

    /// Convert to ServerProperties
    pub fn to_properties(&self) -> ServerProperties {
        let mut props = ServerProperties::new();
        
        // Set basic server properties
        props.set_server_ip(&self.bind_address.ip().to_string());
        props.set_server_port(self.bind_address.port());
        props.set_max_players(self.max_players);
        props.set_motd(&self.motd);
        props.set_online_mode(self.online_mode);
        props.set_view_distance(self.view_distance);
        props.set_simulation_distance(self.simulation_distance);
        
        if let Some(threshold) = self.compression_threshold {
            props.set_network_compression_threshold(threshold as i32);
        } else {
            props.set_network_compression_threshold(-1);
        }

        props
    }

    /// Save configuration to server.properties file
    pub fn save_properties_file<P: AsRef<Path>>(&self, path: P) -> Result<(), ServerError> {
        let props = self.to_properties();
        props.save_to_file(path)
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

    /// Set view distance
    pub fn with_view_distance(mut self, distance: u8) -> Self {
        self.view_distance = distance;
        self
    }

    /// Set simulation distance
    pub fn with_simulation_distance(mut self, distance: u8) -> Self {
        self.simulation_distance = distance;
        self
    }
}
