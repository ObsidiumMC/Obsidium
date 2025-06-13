//! Server listener
//!
//! This module handles accepting new connections and managing the server socket.

use crate::config::ServerConfig;
use crate::error::{ServerError, Result};
use crate::network::Connection;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

/// Server listener that accepts new connections
pub struct ServerListener {
    /// TCP listener
    listener: TcpListener,
    /// Server configuration
    config: ServerConfig,
    /// Channel for sending new connections
    connection_sender: mpsc::UnboundedSender<Connection>,
}

impl ServerListener {
    /// Create a new server listener
    pub async fn new(
        config: ServerConfig,
        connection_sender: mpsc::UnboundedSender<Connection>,
    ) -> Result<Self> {
        let listener = TcpListener::bind(config.bind_address).await?;
        
        Ok(Self {
            listener,
            config,
            connection_sender,
        })
    }
    
    /// Start accepting connections
    pub async fn listen(&self) -> Result<()> {
        loop {
            match self.listener.accept().await {
                Ok((stream, addr)) => {
                    tracing::debug!("New connection from {}", addr);
                    
                    let connection = Connection::new(stream, addr);
                    
                    if let Err(e) = self.connection_sender.send(connection) {
                        tracing::error!("Failed to send connection to handler: {}", e);
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to accept connection: {}", e);
                    // Continue listening despite errors
                }
            }
        }
    }
    
    /// Get the local address the server is bound to
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.listener.local_addr().map_err(ServerError::from)
    }
    
    /// Get the server configuration
    pub fn config(&self) -> &ServerConfig {
        &self.config
    }
}
