//! Error handling for Obsidium

use thiserror::Error;

/// Main error type for the server
#[derive(Error, Debug)]
pub enum ServerError {
    /// IO error
    #[error("Network error: {0}")]
    Io(#[from] std::io::Error),

    /// Protocol error
    #[error("Protocol error: {0}")]
    Protocol(String),

    /// Compression error
    #[error("Compression error: {0}")]
    Compression(#[from] flate2::CompressError),

    /// Decompression error
    #[error("Decompression error: {0}")]
    Decompression(#[from] flate2::DecompressError),
}

/// Convenience type alias
pub type Result<T> = std::result::Result<T, ServerError>;
