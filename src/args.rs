use std::path::PathBuf;
use structopt::StructOpt;

use crate::chunk_type::ChunkType;

#[derive(Debug, StructOpt, PartialEq)]
/// Add secret message in PNG file
pub struct Encode {
    /// File path of PNG file
    #[structopt(parse(from_os_str))]
    pub filepath: PathBuf,

    /// Chunk type for message's chunk
    pub chunk_type: ChunkType,

    /// Message to be encoded in PNG file
    pub message: String,

    /// Optional - file path for output file
    #[structopt(parse(from_os_str))]
    pub output_file: Option<PathBuf>
}

#[derive(Debug, StructOpt, PartialEq)]
/// Show hidden message in PNG file
pub struct Decode {
    /// File path of PNG file
    #[structopt(parse(from_os_str))]
    pub filepath: PathBuf,

    /// Chunk type of chunk that we want to decode
    pub chunk_type: ChunkType
}

#[derive(Debug, StructOpt, PartialEq)]
/// Remove hidden message from PNG file
pub struct Remove {
    /// File path of PNG file
    #[structopt(parse(from_os_str))]
    pub filepath: PathBuf,

    /// Chunk type of chunk that we want to remove
    pub chunk_type: ChunkType
}

#[derive(Debug, StructOpt, PartialEq)]
/// Print every chunk of PNG file
pub struct Print {
    /// File path of output file
    #[structopt(parse(from_os_str))]
    pub filepath: PathBuf,
}

#[derive(Debug, StructOpt, PartialEq)]
#[structopt(name = "subcommand", about = "Pngme subcommands for command line")]
pub enum Subcommand {
    /// Add a secret message to a PNG file
    Encode(Encode),
    /// Show a secret message from a PNG file
    Decode(Decode),
    /// Remove a secret message from a PNG file
    Remove(Remove),
    /// Print every chunk from a PNG file
    Print(Print)
}

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_encode() {
        let expected = Subcommand::Encode(Encode {
            filepath: PathBuf::from("./dice.png"),
            chunk_type: ChunkType::from_str("ruSt").unwrap(),
            message: String::from("This is a test"),
            output_file: None
        });

        let opt = Opt::from_iter(vec![
            "pngme", 
            "encode", 
            "./dice.png", 
            "ruSt", 
            "This is a test"
        ]);

        let actual = opt.subcommand;
        println!("{:?}", actual);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_encode_with_output_file() {
        let expected = Subcommand::Encode(Encode {
            filepath: PathBuf::from("./dice.png"),
            chunk_type: ChunkType::from_str("ruSt").unwrap(),
            message: String::from("This is a test"),
            output_file: Some(PathBuf::from("./output.png"))
        });

        let opt = Opt::from_iter(vec![
            "pngme", 
            "encode", 
            "./dice.png", 
            "ruSt", 
            "This is a test",
            "./output.png"
        ]);

        let actual = opt.subcommand;
        println!("{:?}", actual);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_decode() {
        let expected = Subcommand::Decode(Decode {
            filepath: PathBuf::from("./dice.png"),
            chunk_type: ChunkType::from_str("ruSt").unwrap(),
        });

        let opt = Opt::from_iter(vec![
            "pngme", 
            "decode", 
            "./dice.png", 
            "ruSt"
        ]);

        let actual = opt.subcommand;
        println!("{:?}", actual);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_remove() {
        let expected = Subcommand::Remove(Remove {
            filepath: PathBuf::from("./dice.png"),
            chunk_type: ChunkType::from_str("ruSt").unwrap(),
        });

        let opt = Opt::from_iter(vec![
            "pngme", 
            "remove", 
            "./dice.png", 
            "ruSt"
        ]);

        let actual = opt.subcommand;
        println!("{:?}", actual);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_print() {
        let expected = Subcommand::Print(Print {
            filepath: PathBuf::from("./output.png")
        });

        let opt = Opt::from_iter(vec![
            "pngme", 
            "print", 
            "./output.png"
        ]);

        let actual = opt.subcommand;
        println!("{:?}", actual);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_unknown_subcommand() {
        let result = Opt::from_iter_safe(vec!["pngme", "add", "./dice.png"]);

        assert!(result.is_err());
    }
}