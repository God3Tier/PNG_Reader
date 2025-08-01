mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn testing_chunks() -> Vec<chunk::Chunk> {
    vec![
        chunk_from_strings("FrSt", "I am the first chunk").unwrap(),
        chunk_from_strings("miDl", "I am another chunk").unwrap(),
        chunk_from_strings("LASt", "I am the last chunk").unwrap(),
    ]
}
fn testing_png() -> png::Png {
    let chunks = testing_chunks();
    let png = png::Png::from_chunks(chunks);
    // println!("png success");
    return png
}

fn chunk_from_strings(chunk_type: &str, data: &str) -> Result<chunk::Chunk> {
    use std::str::FromStr;

    let chunk_type = chunk_type::ChunkType::from_str(chunk_type)?;
    let data: Vec<u8> = data.bytes().collect();
    Ok(chunk::Chunk::new(chunk_type, data))
}

// fn print_png() {
//     for x in PNG_FILE {
//         if !(x >= 65 && x <= 90) && !(x >= 97 && x <= 122) {
//             print!("{}", x.to_string());
//         }
    
//         print!("{}", x as char);
//     }
// }

fn test_chunk_types() {
    let bytes:[u8; 4] = "FrST".as_bytes().try_into().unwrap();
    chunk_type::ChunkType::try_from(bytes);
    println!("Valid 1 ");
    let bytes:[u8; 4] = "miDl".as_bytes().try_into().unwrap();
    chunk_type::ChunkType::try_from(bytes);
    println!("Valid 2 ");
    let bytes:[u8; 4] = "LASt".as_bytes().try_into().unwrap();
    chunk_type::ChunkType::try_from(bytes);
    println!("Valid");
}

fn main() -> Result<()> {
    // test_chunk_types();
    testing_png();
    Ok({})
}