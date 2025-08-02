use std::env::{self, args};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let input: Vec<String> = args().collect();
    
    for s in &input {
        println!("{s}");
    }
    
    match input[1].to_lowercase().as_str() {
        "encode" =>  {
            if input.len() < 5 {
                return Err("Not enough arguments".into());
            }
            let mut args = args::Args::new(input[2].as_str(), args::PngArgs::Encode(input[3].clone(), input[4].clone()));
            match args.encode() {
                Ok(_) => println!("Message encoded successfully"),
                Err(e) => return Err(format!("Unable to encode message because of {}", e).into())
            }
        }, 
        "decode" => {
            if input.len() < 4 {
                return Err("Not enough arguments".into());
            }
            let mut args = args::Args::new(input[2].as_str(), args::PngArgs::Decode(input[3].clone()));
            match args.encode() {
                Ok(message) => println!("Message decoded successfully: Message is \n {:?}", message),
                Err(e) => return Err(format!("Unable to decode message because of {}", e).into())
            }
        }, 
        "delete" => {
            if input.len() < 4 {
                return Err("Not enough arguments".into());
            }
            let mut args = args::Args::new(input[2].as_str(), args::PngArgs::Delete(input[3].clone()));
            match args.encode() {
                Ok(_) => println!("Message deleted successfully"),
                Err(e) => return Err(format!("Unable to delete message because of {}", e).into())
            }
        }, 
        "print" => {
            if input.len() < 3 {
                return Err("Not enough arguments".into());
            }
            let args = args::Args::new(input[2].as_str(), args::PngArgs::Print());
            args.print();
        }, 
        _ => return Err("Invalid command".into())
    }
    Ok({})
}