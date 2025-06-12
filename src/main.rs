//! Obsidium Minecraft Server
//!
//! A high-performance Minecraft server written in Rust.

#![deny(
    clippy::too_many_lines,
    missing_docs,
    clippy::uninlined_format_args,
    clippy::panic,
    unused_variables,
    unused_imports,
    unused_must_use
)]

mod error;
mod logger;

use error::ServerError;
use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    // Initialize logger
    logger::init();

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
            Ok((mut socket, addr)) => {
                tracing::debug!("New connection from {}", addr);

                tokio::spawn(async move {
                    let mut buffer = [0; 1024];
                    match socket.read(&mut buffer).await {
                        Ok(0) => {
                            tracing::debug!("Connection closed by peer");
                        }
                        Ok(n) => {
                            tracing::debug!("From ({addr}) received: {:?}", &buffer[..n]);
                        }
                        Err(e) => {
                            tracing::error!("Failed to read from socket: {}", e);
                        }
                    }
                });
            }
            Err(e) => {
                tracing::error!("Failed to accept connection: {}", e);
            }
        }
    }
}
