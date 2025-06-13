//! Packet compression implementation
//!
//! This module handles zlib compression and decompression of Minecraft packets
//! according to the protocol specification.

use crate::error::{Result, ServerError};
use crate::protocol::types::VarInt;
use flate2::{
    Compress, Compression as FlateCompression, Decompress, FlushCompress, FlushDecompress, Status,
};

/// Compression utilities for Minecraft packets
pub struct Compression {
    /// Zlib compressor
    compressor: Compress,
    /// Zlib decompressor
    decompressor: Decompress,
    /// Compression threshold
    threshold: u32,
}

impl Compression {
    /// Create a new compression instance
    pub fn new(threshold: u32) -> Self {
        Self {
            compressor: Compress::new(FlateCompression::default(), false),
            decompressor: Decompress::new(false),
            threshold,
        }
    }

    /// Compress packet data if it exceeds the threshold
    pub fn compress_packet(&mut self, packet_id: VarInt, data: &[u8]) -> Result<Vec<u8>> {
        // Calculate uncompressed length (packet ID + data)
        let mut uncompressed_data = Vec::new();
        packet_id.write(&mut uncompressed_data)?;
        uncompressed_data.extend_from_slice(data);

        let uncompressed_length = uncompressed_data.len();

        // Validate uncompressed length against protocol limits
        if uncompressed_length > crate::protocol::MAX_UNCOMPRESSED_PACKET_SIZE {
            return Err(ServerError::Protocol(format!(
                "Uncompressed packet too large: {} > {}",
                uncompressed_length,
                crate::protocol::MAX_UNCOMPRESSED_PACKET_SIZE
            )));
        }

        // If below threshold, send uncompressed
        if uncompressed_length < self.threshold as usize {
            let mut result = Vec::new();

            // Data Length (0 for uncompressed)
            VarInt(0).write(&mut result)?;

            // Packet ID + Data
            result.extend_from_slice(&uncompressed_data);

            return Ok(result);
        }

        // Compress the data
        let mut compressed_data = Vec::with_capacity(uncompressed_length);

        self.compressor.reset();

        let mut input_pos = 0;
        let mut output_pos = 0;

        loop {
            let old_input_pos = self.compressor.total_in() as usize;
            let old_output_pos = self.compressor.total_out() as usize;

            // Ensure we have space in the output buffer
            if compressed_data.len() < output_pos + 1024 {
                compressed_data.resize(output_pos + 1024, 0);
            }

            let status = self.compressor.compress(
                &uncompressed_data[input_pos..],
                &mut compressed_data[output_pos..],
                if input_pos == uncompressed_length {
                    FlushCompress::Finish
                } else {
                    FlushCompress::None
                },
            )?;

            let new_input_pos = self.compressor.total_in() as usize;
            let new_output_pos = self.compressor.total_out() as usize;

            input_pos += new_input_pos - old_input_pos;
            output_pos += new_output_pos - old_output_pos;

            match status {
                Status::Ok => {
                    if input_pos >= uncompressed_length {
                        break;
                    }
                }
                Status::StreamEnd => break,
                Status::BufError => {
                    // Need more output space
                    compressed_data.resize(compressed_data.len() + 1024, 0);
                }
            }
        }

        compressed_data.truncate(output_pos);

        // Build final packet
        let mut result = Vec::new();

        // Data Length (uncompressed length)
        VarInt(uncompressed_length as i32).write(&mut result)?;

        // Compressed data
        result.extend_from_slice(&compressed_data);

        // Validate final packet size
        if result.len() > crate::protocol::MAX_PACKET_SIZE {
            return Err(ServerError::Protocol(format!(
                "Compressed packet too large: {} > {}",
                result.len(),
                crate::protocol::MAX_PACKET_SIZE
            )));
        }

        Ok(result)
    }

    /// Decompress packet data
    pub fn decompress_packet(&mut self, data: &[u8]) -> Result<(VarInt, Vec<u8>)> {
        let mut cursor = std::io::Cursor::new(data);

        // Read data length
        let data_length = VarInt::read(&mut cursor)?;

        if data_length.0 < 0 {
            return Err(ServerError::Protocol(
                "Negative data length in compressed packet".to_string(),
            ));
        }

        let compressed_data = &data[cursor.position() as usize..];

        // If data length is 0, packet is uncompressed
        if data_length.0 == 0 {
            // Vanilla server rejects compressed packets smaller than threshold
            if compressed_data.len() >= self.threshold as usize {
                return Err(ServerError::Protocol(
                    "Uncompressed packet marked as compressed exceeds threshold".to_string(),
                ));
            }

            let mut uncompressed_cursor = std::io::Cursor::new(compressed_data);
            let packet_id = VarInt::read(&mut uncompressed_cursor)?;
            let remaining_data =
                compressed_data[uncompressed_cursor.position() as usize..].to_vec();
            return Ok((packet_id, remaining_data));
        }

        // Decompress the data
        let uncompressed_length = data_length.0 as usize;

        // Validate uncompressed length
        if uncompressed_length > crate::protocol::MAX_UNCOMPRESSED_PACKET_SIZE {
            return Err(ServerError::Protocol(format!(
                "Uncompressed length too large: {} > {}",
                uncompressed_length,
                crate::protocol::MAX_UNCOMPRESSED_PACKET_SIZE
            )));
        }

        // Check that compressed packet should be compressed (>= threshold)
        if uncompressed_length < self.threshold as usize {
            return Err(ServerError::Protocol(format!(
                "Compressed packet below threshold: {} < {}",
                uncompressed_length, self.threshold
            )));
        }

        let mut uncompressed_data = vec![0u8; uncompressed_length];

        self.decompressor.reset(false);

        let mut input_pos = 0;
        let mut output_pos = 0;

        loop {
            let old_input_pos = self.decompressor.total_in() as usize;
            let old_output_pos = self.decompressor.total_out() as usize;

            let status = self.decompressor.decompress(
                &compressed_data[input_pos..],
                &mut uncompressed_data[output_pos..],
                FlushDecompress::None,
            )?;

            let new_input_pos = self.decompressor.total_in() as usize;
            let new_output_pos = self.decompressor.total_out() as usize;

            input_pos += new_input_pos - old_input_pos;
            output_pos += new_output_pos - old_output_pos;

            match status {
                Status::Ok => {
                    if output_pos >= uncompressed_length {
                        break;
                    }
                }
                Status::StreamEnd => break,
                Status::BufError => {
                    return Err(ServerError::Protocol(
                        "Decompression buffer error".to_string(),
                    ));
                }
            }
        }

        if output_pos != uncompressed_length {
            return Err(ServerError::Protocol(format!(
                "Decompressed length mismatch: expected {}, got {}",
                uncompressed_length, output_pos
            )));
        }

        // Parse packet ID from decompressed data
        let mut uncompressed_cursor = std::io::Cursor::new(&uncompressed_data);
        let packet_id = VarInt::read(&mut uncompressed_cursor)?;
        let remaining_data = uncompressed_data[uncompressed_cursor.position() as usize..].to_vec();

        Ok((packet_id, remaining_data))
    }

    /// Update compression threshold
    pub fn set_threshold(&mut self, threshold: u32) {
        self.threshold = threshold;
    }

    /// Get current compression threshold
    pub fn threshold(&self) -> u32 {
        self.threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_uncompressed() {
        // Test small packet (uncompressed path)
        let mut compression = Compression::new(64);
        let packet_id = VarInt(0x42);
        let data = vec![0x01, 0x02, 0x03]; // Small data that won't be compressed

        let compressed = compression.compress_packet(packet_id, &data).unwrap();
        let (decoded_id, decoded_data) = compression.decompress_packet(&compressed).unwrap();

        assert_eq!(packet_id, decoded_id);
        assert_eq!(data, decoded_data);
    }
}
