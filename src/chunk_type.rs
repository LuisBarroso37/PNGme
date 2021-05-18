use std::convert::TryFrom;
use std::error;
use std::fmt::{self, Display};
use std::str::{self, FromStr};

use crate::{Error, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

/// Chunk Type for v1.2 of the PNG spec
///
/// See [PNG Structure - Chunk Naming Conventions](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html#Chunk-naming-conventions) for details
impl ChunkType {
    /// Bytes encoding the chunk type
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    /// Bytes must only be in the lower-case and upper-case ASCII ranges, and the reserved bit must be valid
    pub fn is_valid(&self) -> bool {
        let valid_chars = self
            .bytes
            .iter()
            .all(|b| (65..=90).contains(b) || (97..=122).contains(b));

        valid_chars && self.is_reserved_bit_valid()
    }

    /// A type code is critical if bit 5 (value 32) of the first byte is 0
    pub fn is_critical(&self) -> bool {
        (self.bytes[0] & 32) == 0
    }

    /// A type code is public if bit 5 (value 32) of the second byte is 0
    pub fn is_public(&self) -> bool {
        (self.bytes[1] & 32) == 0
    }

    /// Bit 5 of the third byte is reserved and must be 0
    pub fn is_reserved_bit_valid(&self) -> bool {
        (self.bytes[2] & 32) == 0
    }

    /// A type code is safe to copy if bit 5 (value 32) of the fourth byte is 1
    pub fn is_safe_to_copy(&self) -> bool {
        (self.bytes[3] & 32) != 0
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self> {
        Ok(ChunkType { bytes: value })
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // Get bytes from string input
        let bytes = s.as_bytes();

        // Throw error if input has less than 4 bytes
        if bytes.len() != 4 {
            return Err(Box::new(ChunkTypeError::ByteLengthError(bytes.len())));
        }

        // Check if all chars in the input for valid chunk type codes
        let valid_chars = bytes
            .iter()
            .all(|b| (65..=90).contains(b) || (97..=122).contains(b));

        if !valid_chars {
            return Err(Box::new(ChunkTypeError::InvalidCharacter));
        }

        // Create byte array from input
        let sized: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];

        ChunkType::try_from(sized)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            std::str::from_utf8(&self.bytes).map_err(|_| fmt::Error)?
        )
    }
}

/// Chunk type errors
#[derive(Debug)]
pub enum ChunkTypeError {
    /// Chunk type has incorrect number of bytes (4 expected)
    ByteLengthError(usize),

    /// The input string contains one or more invalid characters
    InvalidCharacter,
}

impl error::Error for ChunkTypeError {}

impl Display for ChunkTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ChunkTypeError::ByteLengthError(actual) => write!(
                f,
                "Expected 4 bytes but received {} when creating chunk type",
                actual
            ),
            ChunkTypeError::InvalidCharacter => {
                write!(f, "Input contains one or more invalid characters")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
