use crc::crc32;
use std::fmt::{self, Display};
use std::convert::{TryFrom, TryInto};
use std::str;
use std::error;

use crate::{Error, Result};
use crate::chunk_type::ChunkType;

/// Represents a single chunk in the PNG spec
#[derive(Debug, PartialEq, Eq)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32
}

impl Chunk {
    /// Create new chunk
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let length = data.len() as u32;

        let chunk_data: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();
        let crc = crc32::checksum_ieee(&chunk_data);

        Self {
            length,
            chunk_type,
            data,
            crc
        }
    }

    /// Length of the chunk
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Chunk type
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    /// Chunk data
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// CRC of the entire chunk
    pub fn crc(&self) -> u32 {
        self.crc
    }

    /// Chunk data as a string
    pub fn data_as_string(&self) -> Result<String> {
        match str::from_utf8(&self.data) {
            Ok(s) => Ok(s.to_string()),
            Err(e) => Err(Box::new(e))
        }
    }

    /// Entire chunk represented as bytes
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
        .to_be_bytes()
        .iter()
        .chain(self.chunk_type.bytes().iter())
        .chain(self.data.iter())
        .chain(self.crc.to_be_bytes().iter())
        .copied()
        .collect::<Vec<u8>>()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        // Throw error if bytes has less than the necessary chunk metadata
        if bytes.len() < 12 {
            return Err(Box::from(ChunkError::InputTooSmall));
        }

        // Get first 4 bytes which correspond to the chunk's data length
        let (data_length, bytes) = bytes.split_at(4);
        let length = u32::from_be_bytes(data_length.try_into()?);

        // Get next 4 bytes which correspond to the chunk's type
        let (chunk_type_bytes, bytes) = bytes.split_at(4);

        let chunk_type_bytes: [u8; 4] = chunk_type_bytes.try_into()?;
        let chunk_type = ChunkType::try_from(chunk_type_bytes)?;

        if !chunk_type.is_valid() {
            return Err(Box::from(ChunkError::InvalidChunkType));
        }

        // Get chunk's data and crc from remaining bytes
        // length refers to the chunk's data length
        let (data, bytes) = bytes.split_at(length as usize);
        let (crc, _) = bytes.split_at(4);

        let data: Vec<u8> = data.try_into()?;
        let crc = u32::from_be_bytes(crc.try_into()?);
        
        // Calculate crc from chunk's type and chunk's data
        let chunk_data: Vec<u8> = chunk_type.bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();

        let actual_crc = crc32::checksum_ieee(&chunk_data);
        let expected_crc = crc;

        if actual_crc != expected_crc {
            return Err(Box::new(ChunkError::InvalidCrc(expected_crc, actual_crc)));
        }
        
        Ok(Chunk {
            length,
            chunk_type,
            data,
            crc
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            "length: {}, chunk type: {}, data: {:?}, crc: {:?}",
            self.length,
            self.chunk_type,
            self.data,
            self.crc
        )
    }
}

#[derive(Debug)]
pub enum ChunkError {
    /// Input bytes length is smaller than the necessary 12 bytes for chunk's metadata
    InputTooSmall,

    /// Invalid crc for chunk
    InvalidCrc(u32, u32),

    /// Invalid chunk type
    InvalidChunkType
}

impl error::Error for ChunkError {}

impl Display for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChunkError::InputTooSmall => {
                write!(f, "At least 12 bytes must be supplied to construct a chunk")
            },
            ChunkError::InvalidCrc(expected, actual) => {
                write!(
                    f,
                    "Invalid CRC when constructing chunk. Expected {} but found {}",
                    expected, actual
                )
            },
            ChunkError::InvalidChunkType => write!(f, "Invalid chunk type")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        println!("{:?}", chunk_data);
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}