//! Obsidium Minecraft Server
//!
//! A high-performance Minecraft server written in Rust.

#![deny(clippy::too_many_lines, missing_docs, clippy::panic)]

use obsidium::Result;
use obsidium::config::ServerConfig;
use obsidium::logger;
use obsidium::server::MinecraftServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    logger::init();

    // Create server configuration
    let config = ServerConfig::new()
        .with_motd("A high-performance Minecraft server written in Rust.".to_string())
        .with_max_players(999_999_999)
        .with_compression_threshold(None) // Disable compression for now
        .with_debug(
            std::env::var("RUST_LOG")
                .unwrap_or_default()
                .contains("debug"),
        );

    // Create and run server
    let server = MinecraftServer::new(config).await?;
    server.run().await?;

    Ok(())
}
