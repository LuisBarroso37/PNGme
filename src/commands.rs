use std::fs;
use std::convert::TryFrom;
use structopt::StructOpt;

use crate::args::*;
use crate::png;
use crate::chunk;
use crate::Result;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: Encode) -> Result<()> {
    let Encode { filepath, chunk_type, message, output_file} = args;

    // Read PNG file to vector of bytes
    let bytes = fs::read(&filepath)?;

    // Convert bytes array into png struct
    let mut png = png::Png::try_from(&bytes[..])?;

    // Create chunk from chunk_type and message
    let data: Vec<u8> = message.as_bytes().to_vec();
    let chunk = chunk::Chunk::new(chunk_type, data);

    // Append chunk to png struct
    png.append_chunk(chunk);

    // Write updated png file to a specific output file or
    // overwrite original file
    match output_file {
        Some(path) => fs::write(path, png.as_bytes())?,
        None => fs::write(&filepath, png.as_bytes())?
    }

    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: Decode) -> Result<()> {
    let Decode { filepath, chunk_type} = args;

    // Read PNG file to vector of bytes
    let bytes = fs::read(&filepath)?;

    // Convert bytes array into png struct
    let png = png::Png::try_from(&bytes[..])?;

    // Show chunk if it exists in png
    match png.chunk_by_type(&chunk_type.to_string()) {
        Some(chunk) => {
            println!("{}", chunk);
            Ok(())
        },
        None => Err("Could not find chunk".into())
    }
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: Remove) -> Result<()> {
    let Remove { filepath, chunk_type} = args;
    // Read PNG file to vector of bytes
    let bytes = fs::read(&filepath)?;

    // Convert bytes array into png struct
    let mut png = png::Png::try_from(&bytes[..])?;

    // Remove chunk if it exists in png struct
    let chunk = png.remove_chunk(&chunk_type.to_string())?;

    // Overwrite PNG file with updated version
    fs::write(&filepath, png.as_bytes())?;

    println!("Removed chunk: {}", chunk);
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: Print) -> Result<()> {
    let Print { filepath} = args;
    // Read PNG file to vector of bytes
    let bytes = fs::read(&filepath)?;

    // Convert bytes array into png struct
    let png = png::Png::try_from(&bytes[..])?;

    for chunk in png.chunks() {
        println!("{}", chunk);
    }

    Ok(())
}

pub fn run(subcommand: Subcommand) -> Result<()> {
    match subcommand {
        Subcommand::Encode(args) => encode(args),
        Subcommand::Decode(args) => decode(args),
        Subcommand::Remove(args) => remove(args),
        Subcommand::Print(args) => print_chunks(args)
    }
}