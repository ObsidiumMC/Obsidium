//! Favicon utilities for server icon support
//!
//! This module provides utilities for loading and encoding server favicons
//! in the format required by the Minecraft protocol.

use crate::error::{Result, ServerError};
use base64::Engine;
use std::fs;
use std::path::Path;

/// Load a favicon from a file path and encode it as a data URL
///
/// The image must be a 64x64 PNG file. Returns a base64-encoded data URL
/// in the format: `data:image/png;base64,<encoded-data>`
pub fn load_favicon_from_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let path = path.as_ref();

    // Validate file extension
    if let Some(ext) = path.extension() {
        if ext.to_string_lossy().to_lowercase() != "png" {
            return Err(ServerError::Protocol(
                "Favicon must be a PNG file".to_string(),
            ));
        }
    } else {
        return Err(ServerError::Protocol(
            "Favicon file must have .png extension".to_string(),
        ));
    }

    // Read the file
    let image_data = fs::read(path)
        .map_err(|e| ServerError::Protocol(format!("Failed to read favicon file: {}", e)))?;
    // Check file size before processing
    tracing::debug!("Favicon file size: {} bytes", image_data.len());

    // Check if file size is suitable
    let max_size = max_recommended_file_size();
    if !is_suitable_file_size(image_data.len()) {
        return Err(ServerError::Protocol(format!(
            "Favicon file is too large ({} bytes). Maximum recommended size is {} bytes. Please optimize your PNG file.",
            image_data.len(),
            max_size
        )));
    }

    // Validate PNG header
    if !is_valid_png(&image_data) {
        return Err(ServerError::Protocol("Invalid PNG file format".to_string()));
    }

    // Encode as base64 data URL
    let encoded = base64::engine::general_purpose::STANDARD.encode(&image_data);
    let data_url = format!("data:image/png;base64,{}", encoded);

    // Check if the final data URL will fit in Minecraft's string limit
    if data_url.len() > crate::protocol::types::McString::MAX_LENGTH {
        return Err(ServerError::Protocol(format!(
            "Favicon data URL is too large ({} chars) for Minecraft protocol (max: {} chars). Please use a smaller/more optimized PNG file.",
            data_url.len(),
            crate::protocol::types::McString::MAX_LENGTH
        )));
    }

    Ok(data_url)
}

/// Create a favicon data URL from raw PNG data
pub fn create_favicon_from_data(png_data: &[u8]) -> Result<String> {
    if !is_valid_png(png_data) {
        return Err(ServerError::Protocol("Invalid PNG data".to_string()));
    }

    let encoded = base64::engine::general_purpose::STANDARD.encode(png_data);
    Ok(format!("data:image/png;base64,{}", encoded))
}

/// Validate PNG file signature
fn is_valid_png(data: &[u8]) -> bool {
    const PNG_SIGNATURE: &[u8] = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    data.len() >= PNG_SIGNATURE.len() && &data[..PNG_SIGNATURE.len()] == PNG_SIGNATURE
}

/// Generate a simple default favicon (placeholder)
/// Returns a base64-encoded 64x64 PNG with a simple design
pub fn generate_default_favicon() -> String {
    // This is a minimal 64x64 PNG (just the PNG header + minimal data)
    // A proper 64x64 PNG would be much larger, but this serves as a placeholder
    // You should replace this with actual PNG data for a real favicon
    let minimal_png_data = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        // Minimal PNG structure - this won't render properly but won't crash clients
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, //IHDR chunk start
        0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x40, // 64x64 dimensions
        0x08, 0x02, 0x00, 0x00, 0x00, 0x25, 0x0B, 0xE6, 0x89, // IHDR data + CRC
    ];

    let encoded = base64::engine::general_purpose::STANDARD.encode(&minimal_png_data);
    format!("data:image/png;base64,{}", encoded)
}

/// Get the maximum recommended file size for a favicon
/// This ensures the base64-encoded result will fit within Minecraft's limits
pub fn max_recommended_file_size() -> usize {
    // Base64 encoding increases size by ~33%, plus we need room for the data URL prefix
    // Conservative estimate: leave room for the "data:image/png;base64," prefix (26 chars)
    let available_chars = crate::protocol::types::McString::MAX_LENGTH - 26;

    // Base64 uses 4 chars for every 3 bytes, so max bytes = available_chars * 3 / 4
    (available_chars * 3) / 4
}

/// Check if a PNG file size is suitable for use as a favicon
pub fn is_suitable_file_size(file_size: usize) -> bool {
    file_size <= max_recommended_file_size()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_png_validation() {
        let valid_png_header = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        assert!(is_valid_png(&valid_png_header));

        let invalid_data = [0x00, 0x01, 0x02, 0x03];
        assert!(!is_valid_png(&invalid_data));
    }

    #[test]
    fn test_create_favicon_from_data() {
        let valid_png_data = [
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A,
            0x0A, // PNG signature
                 // ... (rest would be actual PNG data)
        ];

        let result = create_favicon_from_data(&valid_png_data);
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("data:image/png;base64,"));
    }
}
