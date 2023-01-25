use crate::{Result, Error};
use crate::png::Png;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;

use std::fs::{read, write};
use std::str::FromStr;


pub fn encode(path: &String, chunk_type: &String, message: &String) -> Result<()>{
    let file_bytes = read(&path)?;
    let mut png = Png::try_from(&file_bytes[..])?;
    let chunk_type = ChunkType::from_str(&chunk_type)?;
    let data = message.clone().into_bytes();
    let chunk = Chunk::new(chunk_type, data);
    png.append_chunk(chunk);
    write(&path, png.as_bytes())?;
    Ok(())
}

pub fn decode(path: &String, chunk_type: &String) -> Result<()>{
    let file_bytes = read(&path)?;
    let png = Png::try_from(&file_bytes[..])?;
    let message = png.chunk_by_type(chunk_type);
    match message {
        Some(chunk) => println!("Found Message: {}", chunk.data_as_string().unwrap()),
        None => return Err("No message found!")?,
    }
    Ok(())
}

pub fn remove(path: &String, chunk_type: &String) -> Result<()>{
    let file_bytes = read(&path)?;
    let mut png = Png::try_from(&file_bytes[..])?;
    let chunk = png.remove_chunk(chunk_type)?;
    println!("Removed chunk: {chunk}");
    write(&path, png.as_bytes())?;
    Ok(())
}

pub fn print(path: &String) -> Result<()>{
    let file_bytes = read(&path)?;
    let png = Png::try_from(&file_bytes[..])?;
    println!("Opened {path} and constructed: {png}!");
    Ok(())
}