//! Connection state management
//!
//! This module handles the different states a Minecraft connection can be in
//! and transitions between them.

/// Represents the current state of a Minecraft connection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    /// Initial handshaking state
    Handshaking,
    /// Server status (ping) state
    Status,
    /// Login/authentication state
    Login,
    /// Active gameplay state
    Play,
}

impl ConnectionState {
    /// Check if packets can be compressed in this state
    pub fn allows_compression(&self) -> bool {
        matches!(self, ConnectionState::Login | ConnectionState::Play)
    }
    
    /// Get the string representation of the state
    pub fn as_str(&self) -> &'static str {
        match self {
            ConnectionState::Handshaking => "handshaking",
            ConnectionState::Status => "status",
            ConnectionState::Login => "login", 
            ConnectionState::Play => "play",
        }
    }
}

impl Default for ConnectionState {
    fn default() -> Self {
        ConnectionState::Handshaking
    }
}

/// Protocol state management for a connection
#[derive(Debug)]
pub struct ProtocolState {
    /// Current connection state
    pub state: ConnectionState,
    /// Whether compression is enabled
    pub compression_enabled: bool,
    /// Compression threshold in bytes
    pub compression_threshold: Option<u32>,
    /// Protocol version negotiated with client
    pub protocol_version: Option<i32>,
}

impl ProtocolState {
    /// Create a new protocol state
    pub fn new() -> Self {
        Self {
            state: ConnectionState::default(),
            compression_enabled: false,
            compression_threshold: None,
            protocol_version: None,
        }
    }
    
    /// Transition to a new state
    pub fn transition_to(&mut self, new_state: ConnectionState) {
        tracing::debug!(
            "Connection state transition: {} -> {}",
            self.state.as_str(),
            new_state.as_str()
        );
        self.state = new_state;
    }
    
    /// Enable compression with the given threshold
    pub fn enable_compression(&mut self, threshold: u32) {
        if self.state.allows_compression() {
            self.compression_enabled = true;
            self.compression_threshold = Some(threshold);
            tracing::debug!("Compression enabled with threshold: {}", threshold);
        } else {
            tracing::warn!(
                "Attempted to enable compression in state: {}", 
                self.state.as_str()
            );
        }
    }
    
    /// Disable compression
    pub fn disable_compression(&mut self) {
        self.compression_enabled = false;
        self.compression_threshold = None;
        tracing::debug!("Compression disabled");
    }
    
    /// Set the protocol version
    pub fn set_protocol_version(&mut self, version: i32) {
        self.protocol_version = Some(version);
        tracing::debug!("Protocol version set to: {}", version);
    }
    
    /// Check if a packet should be compressed based on its size
    pub fn should_compress(&self, packet_size: usize) -> bool {
        if !self.compression_enabled {
            return false;
        }
        
        if let Some(threshold) = self.compression_threshold {
            packet_size >= threshold as usize
        } else {
            false
        }
    }
}

impl Default for ProtocolState {
    fn default() -> Self {
        Self::new()
    }
}
