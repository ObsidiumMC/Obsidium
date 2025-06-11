//! VarInt implementation for Minecraft protocol
//! 
//! VarInt is a variable-length integer encoding used extensively in the Minecraft protocol.

use std::io::{Read, Write, Result as IoResult};

/// A variable-length integer as used in the Minecraft protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VarInt(pub i32);

impl VarInt {
    /// Maximum number of bytes a VarInt can use
    pub const MAX_SIZE: usize = 5;
    
    /// Segment bits mask (7 bits)
    const SEGMENT_BITS: u8 = 0x7F;
    
    /// Continue bit (most significant bit)
    const CONTINUE_BIT: u8 = 0x80;

    /// Read a VarInt from a reader
    pub fn read<R: Read>(reader: &mut R) -> IoResult<VarInt> {
        let mut value = 0i32;
        let mut position = 0u32;
        
        loop {
            let mut byte = [0u8; 1];
            reader.read_exact(&mut byte)?;
            let current_byte = byte[0];
            
            value |= ((current_byte & Self::SEGMENT_BITS) as i32) << position;
            
            if (current_byte & Self::CONTINUE_BIT) == 0 {
                break;
            }
            
            position += 7;
            
            if position >= 32 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "VarInt is too big"
                ));
            }
        }
        
        Ok(VarInt(value))
    }
    
    /// Write a VarInt to a writer
    pub fn write<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        let mut value = self.0 as u32;
        
        loop {
            if (value & !Self::SEGMENT_BITS as u32) == 0 {
                writer.write_all(&[value as u8])?;
                return Ok(());
            }
            
            writer.write_all(&[(value & Self::SEGMENT_BITS as u32) as u8 | Self::CONTINUE_BIT])?;
            value >>= 7;
        }
    }
    
    /// Get the number of bytes this VarInt would use when encoded
    pub fn size(&self) -> usize {
        let mut value = self.0 as u32;
        let mut size = 1;
        
        while (value & !Self::SEGMENT_BITS as u32) != 0 {
            size += 1;
            value >>= 7;
        }
        
        size
    }
    
    /// Read a VarInt from a byte slice
    pub fn from_bytes(bytes: &[u8]) -> Result<(VarInt, usize), std::io::Error> {
        let mut value = 0i32;
        let mut position = 0u32;
        let mut bytes_read = 0;
        
        for &byte in bytes {
            bytes_read += 1;
            
            value |= ((byte & Self::SEGMENT_BITS) as i32) << position;
            
            if (byte & Self::CONTINUE_BIT) == 0 {
                return Ok((VarInt(value), bytes_read));
            }
            
            position += 7;
            
            if position >= 32 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "VarInt is too big"
                ));
            }
        }
        
        Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "Incomplete VarInt"
        ))
    }
    
    /// Convert to bytes vector
    pub fn to_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.write(&mut bytes).unwrap();
        bytes
    }
}

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        VarInt(value)
    }
}

impl From<VarInt> for i32 {
    fn from(varint: VarInt) -> Self {
        varint.0
    }
}

impl From<usize> for VarInt {
    fn from(value: usize) -> Self {
        VarInt(value as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_varint_encoding() {
        let test_cases = vec![
            (0, vec![0x00]),
            (1, vec![0x01]),
            (127, vec![0x7f]),
            (128, vec![0x80, 0x01]),
            (255, vec![0xff, 0x01]),
            (25565, vec![0xdd, 0xc7, 0x01]),
        ];

        for (value, expected) in test_cases {
            let varint = VarInt(value);
            let encoded = varint.to_bytes();
            assert_eq!(encoded, expected, "Failed to encode {value}");
            
            let (decoded, size) = VarInt::from_bytes(&encoded).unwrap();
            assert_eq!(decoded.0, value, "Failed to decode {value}");
            assert_eq!(size, encoded.len(), "Size mismatch for {value}");
        }
    }
}
