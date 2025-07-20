use std::{convert::TryFrom, fmt::{Display, Formatter}, str::FromStr};
use crate::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4]
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }
    
    pub fn is_valid(&self) -> bool {
        for i in self.bytes {
            if (i > 90 || i < 65) && (i > 122 || i < 97) {
                return false;
            }
        }
        
        return self.bytes[2] & 0x20 == 0
    }
    
    pub fn is_critical(&self) -> bool {
        return (self.bytes[0] & 0x20) == 0;
    }
    
    pub fn is_public(&self) -> bool {
        return (self.bytes[1] & 0x20) == 0;
    }
    
    pub fn is_reserved_bit_valid(&self) -> bool {
        return (self.bytes[2] & 0x20) == 0
    }
    
    pub fn is_safe_to_copy(&self) -> bool {
        return !((self.bytes[3] & 0x20) == 0)
    }
    
    pub fn to_string(&self) -> String {
        let mut res: String = String::new();
        
        for i in self.bytes {
            res.push(i as char);
        }
        
        res
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    
    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        for b in bytes {
            if !(b >= 65 && b <= 90) && !(b >= 97 && b <= 122) {
                return Err(format!("Not a letter for {b}").into());
            }
        }
        
        if bytes[2] & 0x20 != 0 {
            return Err("Invalid bit placement".into())
        }
        
        return Ok(ChunkType{
            bytes: bytes
        })
    }
}

impl FromStr for ChunkType {
    type Err = Error;
    
    fn from_str(s: &str) -> Result<Self> {
        if s.len() > 4 {
            return Err("Too long".into());
        }
        let mut bytes: [u8; 4] = [0; 4]; 
        let mut indx = 0;
        
        for i in s.as_bytes() {
            if !(*i >= 65 && *i <= 90) && !(*i >= 97 && *i <= 122) {
                return Err(format!("Not a letter for {i:?}").into());
            }
            bytes[indx] = *i;
            indx += 1;
        }
        
        return Ok(ChunkType { bytes: bytes })
    }
}


impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.bytes)
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