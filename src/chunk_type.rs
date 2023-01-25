use std::{str::FromStr, fmt::Display, str::from_utf8};
use crate::{Result, Error};


#[derive(Eq, PartialEq, Debug)]
pub struct ChunkType {
    value: [u8; 4],
}

impl ChunkType{

    pub fn bytes(&self) -> [u8; 4] {
        self.value
    }

    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    fn is_critical(&self) -> bool {
        self.value[0].is_ascii_uppercase()
    }

    fn is_public(&self) -> bool {
        self.value[1].is_ascii_uppercase()
    }

    fn is_reserved_bit_valid(&self) -> bool {
        self.value[2].is_ascii_uppercase()
    }
    
    fn is_safe_to_copy(&self) -> bool {
        self.value[3].is_ascii_lowercase()
    }
}

impl TryFrom<[u8; 4]> for ChunkType{

    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self> {
        if value.len() == 4 {
            Ok(ChunkType {  value })
        } else {
            Err("More than 4 bytes received")?
        }
    }
}

impl FromStr for ChunkType{

    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let bytes: Vec<u8> = s.bytes().collect();
        for byte in &bytes{
            if !byte.is_ascii_alphabetic() {
                return Err("Non-Alphabetical Character Found")?
            }
        };
        let array = bytes.try_into();
        if let Ok(array) = array {
            Ok(ChunkType{value: array})
        } else {
            Err("Failed to convert vector into array")?
        }
    }
}

impl Display for ChunkType{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = from_utf8(&self.value[..]);
        if let Ok(result) = result {
            write!(f, "{}", result)
        } else {
            write!(f, "Failed to convert into UTF-8")
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