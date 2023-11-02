use crate::args::{MyArgs, EncodeCommand, DecodeCommand, RemoveCommand, PrintCommand};
use crate::png::Png;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;   
use std::fs;

pub fn encode(cmd: EncodeCommand) -> crate::Result<()> {
    let tmp_bytes = fs::read(&cmd.file_path)?;
    let mut tmp_png = Png::try_from(tmp_bytes.as_slice())?;
    let tmp_type: ChunkType = cmd.chunk.parse()?;
    let tmp_chunk = Chunk::new(tmp_type, cmd.message.into_bytes());

    tmp_png.append_chunk(tmp_chunk);
    
    match cmd.output {
        Some(path) => {
            fs::write(path, tmp_png.as_bytes())?;
        }
        None => {
            fs::write(cmd.file_path, tmp_png.as_bytes())?;
        }
    }

    Ok(())
}

pub fn decode(cmd: DecodeCommand) -> crate::Result<()>{
    let tmp_bytes = fs::read(&cmd.file_path)?;
    let tmp_png: Png = Png::try_from(tmp_bytes.as_slice())?;

    let tmp_chunk = tmp_png.chunk_by_type(&cmd.chunk_type);

    if let Some(chunk) = tmp_chunk {
        let tmp_str = chunk.data_as_string()?;
        println!("Secret Message: {}", tmp_str);
    } else {
        println!("No secret message");
    }
        
    Ok(())
}

pub fn remove(cmd: RemoveCommand) -> crate::Result<()>{
    let tmp_bytes = fs::read(&cmd.file_path)?;
    let mut tmp_png = Png::try_from(tmp_bytes.as_slice())?;

    tmp_png.remove_chunk(&cmd.chunk_type)?;

    fs::write(cmd.file_path, tmp_png.as_bytes())?;

    Ok(())
}

pub fn print(cmd: PrintCommand) -> crate::Result<()>{
    let tmp_bytes = fs::read(&cmd.file_path)?;
    let tmp_png = Png::try_from(tmp_bytes.as_slice())?;

    println!("{}", tmp_png);

    Ok(())
}

