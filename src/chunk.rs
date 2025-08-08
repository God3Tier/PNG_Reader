use std::{convert::TryFrom, fmt::{Display, Formatter}, panic, str::FromStr};
use crate::{chunk, Error, Result};
use crc::{Crc, CRC_32_ISO_HDLC};
use crate::chunk_type::ChunkType;


pub const CRC_PNG: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

#[derive(PartialEq, Clone)]
pub struct Chunk {
    length: u32, // 4 bytes 
    chunk_type: ChunkType, // 4 bytes 
    chunk_data: Vec<u8>, // explained by length 
    crc: u32, // 4 bytes 
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;
    
    fn try_from(chunk_data: &[u8]) -> Result<Self> {
        let c_type_bytes = chunk_data[4..8].try_into();
        if c_type_bytes.is_err() {
            return Err("Too small".into());
        }
        let c_type_bytes:[u8; 4] = c_type_bytes.unwrap();
        
        let length: u32 = u32::from_be_bytes(chunk_data[0..4].try_into().unwrap());

        match ChunkType::try_from(c_type_bytes) {
            Ok(chunk_type) => {
                let crc = if (12 + length as usize) > chunk_data.len() {
                    let val = chunk_type.bytes().iter()
                        .map(|&x| x)
                        .chain(chunk_data[8..8 + length as usize].iter().map(|&x| x))
                        .collect::<Vec<u8>>();
                    CRC_PNG.checksum(&val)
                } else {

                    u32::from_be_bytes(chunk_data[8 + length as usize..12 + length as usize].try_into().unwrap())
                };
                
                return Ok(Chunk{
                    chunk_type: chunk_type,
                    length: length,
                    chunk_data: chunk_data[8..8 + length as usize].iter().map(|&x| x).collect::<Vec<u8>>(),
                    crc: crc
                })
            },
            
            Err(e) => return Err(format!("Unable to chunktype due to {}", e).into())
        }
        
    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Length = {}, chunk_type = {:?}, chunk_data = {:?}, crc = {}",self.length, self.chunk_type, self.chunk_data, self.crc)
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let val: Vec<u8>  = chunk_type.bytes().iter()
            .map(|&x| x)
            .chain(data.iter().map(|&x| x))
            .collect();
        
        Chunk {
            length: data.len() as u32,
            chunk_type: chunk_type,
            crc: CRC_PNG.checksum(&val),
            chunk_data: data
        }
    }
    
    pub fn length(&self) -> u32 {
        self.length
    }
    
    pub fn chunk_type(&self) -> ChunkType {
        self.chunk_type.clone()
    }
    
    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    }
    
    pub fn crc(&self) -> u32 {
        self.crc
    }
    
    pub fn data_as_string(&self) -> Result<String> {
        match String::from_utf8(self.chunk_data.clone()) {
            Ok(str) => return Ok(str), 
            Err(e) => return Err(format!("Unable to convert to String because of {}", e).into())
        }
    }
    
    pub fn as_bytes(&self) -> Vec<u8> {
        self.chunk_data.clone()
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

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
        
        // println!("{:?}", chunk_type);
        // println!("{:?}", message_bytes.len());
        // println!("{:?}", crc.to_be_bytes().iter().collect::<Vec<&u8>>().len());
        // println!("{:?}", chunk_data);
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap_or_else(|e| {
            println!("Unable to {e:?}");
            panic!("Failure on chunk_type");
        });
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
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
        // println!("No error in creation");
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
