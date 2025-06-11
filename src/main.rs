//! Obsidium Minecraft Server
//! 
//! A high-performance Minecraft server implementation written in Rust.

#![deny(
    clippy::too_many_lines,
    missing_docs,
    clippy::uninlined_format_args,
    clippy::panic,
)]
#![allow(dead_code, unused_variables, unused_imports, unused_must_use)]

mod error;
mod logger;
mod protocol;
mod client;

use error::ServerError;
use client::ClientConnection;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    // Initialize logger
    logger::init_logger();

    tracing::info!("Starting Obsidium Minecraft Server...");
    let listener = match TcpListener::bind("0.0.0.0:25565").await {
        Ok(l) => l,
        Err(e) => {
            tracing::error!("Failed to bind to address: {}", e);
            return Err(ServerError::Io(e));
        }
    };
    tracing::info!("Server is listening on 0.0.0.0:25565");

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                tracing::debug!("New connection from {}", addr);
                
                // Spawn a new task to handle this client
                tokio::spawn(async move {
                    let client = ClientConnection::new(socket, addr);
                    if let Err(e) = client.handle().await {
                        tracing::error!("Error handling client {}: {:?}", addr, e);
                    }
                });
            }
            Err(e) => {
                tracing::error!("Failed to accept connection: {}", e);
            }
        }
    }
}
