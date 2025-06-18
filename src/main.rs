//! Obsidium Minecraft Server
//!
//! A high-performance Minecraft server written in Rust.

#![deny(clippy::too_many_lines, missing_docs, clippy::panic)]

use obsidium::Result;
use obsidium::config::ServerConfig;
use obsidium::error::ServerError;
use obsidium::logger;
use obsidium::server::MinecraftServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    logger::init();

    // Try to load configuration from server.properties file
    let config = match ServerConfig::from_properties_file("server.properties") {
        Ok(config) => {
            tracing::info!("Loaded configuration from server.properties");
            config
        }
        Err(ServerError::Io(ref e)) if e.kind() == std::io::ErrorKind::NotFound => {
            tracing::info!("server.properties not found, creating default configuration");
            let config = ServerConfig::new()
                .with_motd(
                    "Welcome to Obsidium - an experimental Minecraft server written in Rust!"
                        .to_string(),
                )
                .with_max_players(999_999_999)
                .with_compression_threshold(Some(256))
                .with_favicon(Some("server-icon.png".to_string()));

            // Save the default configuration to server.properties
            if let Err(e) = config.save_properties_file("server.properties") {
                tracing::warn!("Failed to save server.properties: {}", e);
            } else {
                tracing::info!("Created default server.properties file");
            }

            config
        }
        Err(e) => {
            tracing::error!("Failed to load server.properties: {}", e);
            tracing::info!("Using default configuration");
            ServerConfig::new()
                .with_motd(
                    "Welcome to Obsidium - an experimental Minecraft server written in Rust!"
                        .to_string(),
                )
                .with_max_players(999_999_999)
                .with_compression_threshold(Some(256))
                .with_favicon(Some("server-icon.png".to_string()))
        }
    };

    // Create and run server
    let server = MinecraftServer::new(config).await?;
    server.run().await?;

    Ok(())
}
