use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

use crate::png::Png;
use crate::Result;
use crate::Error;
use crate::chunk::Chunk;

pub enum PngArgs {
    Encode(String, String),
    Decode(String),
    Delete(String),
    Print()
}

pub struct Args {
    png: Png,
    func: PngArgs, 
    file_path: String, 
}

impl Args {
    pub fn new(file_path: &str, func: PngArgs) -> Args{
        let bytes = fs::read(file_path).unwrap();
        let bytes: &[u8] = &bytes;
        let png = Png::try_from(bytes).unwrap();
        
        Args {
            png: png, 
            func: func,
            file_path: file_path.to_string()
        }
    }
    
    pub fn encode(&mut self)-> Result<()> {
        match &self.func {
            PngArgs::Encode(chunk_type, message) => {
                let bytes = chunk_type.bytes().into_iter()
                    .chain(message.bytes().into_iter())
                    .collect::<Vec<u8>>();
                let bytes: &[u8] = &bytes;
                match Chunk::try_from(bytes) {
                    Ok(chunk) => {
                        let bytes = u32::to_be_bytes(chunk.length()).iter()
                            .map(|&x| x)
                            .chain(chunk.chunk_type().bytes().iter().map(|&x| x))
                            .chain(chunk.as_bytes().iter().map(|&x| x))
                            .chain(u32::to_be_bytes(chunk.crc()).iter().map(|&x| x))
                            .collect::<Vec<u8>>();
                        let bytes: &[u8] = &bytes;
                        let mut file = OpenOptions::new()
                            .append(true)
                            .create(true)
                            .open(self.file_path.clone())
                            .unwrap();
                            
                        file.write_all(bytes).unwrap_or_else(|e| {
                            panic!("Unable to write to file because of {e}");
                        });  
                        self.png.append_chunk(chunk);
                        return Ok(())
                    },
                    Err(e) => return Err(format!("Unable to create chunk because of {}", e).into())
                }
            },
            _ => Err("Incorrect function call".into())
        }
    }
    
    pub fn decode(&self)-> Option<String> {
        match &self.func  {
            PngArgs::Decode(chunk_type) => {
                match self.png.chunk_by_type(chunk_type) {
                    Some(chunk) => {
                        return Some(String::from_utf8(chunk.as_bytes().to_vec()).unwrap());
                    }
                    None => return None
                }
                    
            }, 
            _ => None
        }
    }
    
    pub fn remove(&mut self) -> Result<()>{
        match &self.func  {
            PngArgs::Decode(chunk_type) => {
                match self.png.remove_first_chunk(chunk_type) {
                    Ok(_) => {
                        let mut file = File::create(self.file_path.clone()).unwrap();
                        file.write_all(&self.png.as_bytes()).unwrap_or_else(|e| {
                            panic!("Unable to write to file because of {e}");
                        });
                        return Ok(())
                    },
                    Err(e) => return Err(format!("Unable to find chunk due to {}", e).into())
                }  
            }, 
            _ => Err("Incorrect function call".into())
        }
    }
    
    pub fn print(&self) {
        println!("{:?}", self.png.header());
        for chunk in self.png.chunks() {
            println!("ChunkType : {}, Message: {:?}", chunk.chunk_type(), String::from_utf8(chunk.as_bytes().to_vec()));
        }
    }
}