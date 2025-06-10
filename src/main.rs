mod error;
mod logger;

use error::ServerError;
use tokio::{io::AsyncReadExt, net::TcpListener};

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    // Initialize logger
    logger::init_logger();

    tracing::info!("Server starting...");
    let listener = match TcpListener::bind("0.0.0.0:25565").await {
        Ok(l) => l,
        Err(e) => {
            tracing::error!("Failed to bind to address: {}", e);
            panic!("Failed to bind: {}", e);
        }
    };
    tracing::info!("Server is listening on 0.0.0.0:25565");

    loop {
        match listener.accept().await {
            Ok((mut socket, addr)) => {
                tracing::debug!("New connection from {}", addr);
                tokio::spawn(async move {
                    let mut buffer = [0; 1024]; // Create a buffer to read data into
                    loop {
                        match socket.read(&mut buffer).await {
                            Ok(0) => {
                                tracing::debug!("Connection closed by {}", addr);
                                break; // Connection was closed
                            }
                            Ok(n) => {
                                // Data was read successfully
                                tracing::debug!(
                                    "Received {} bytes from {}: {:?}",
                                    n,
                                    addr,
                                    &buffer[..n]
                                );
                                // Here you would typically parse the Minecraft protocol data
                                // For now, we're just logging the raw bytes
                            }
                            Err(e) => {
                                tracing::error!("Failed to read from socket; err = {:?}", e);
                                return; // End this task on error
                            }
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
