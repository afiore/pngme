use std::path::PathBuf;

use crate::chunk_type::ChunkType;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pngme", about = "The PNG annotation command")]
pub(crate) struct Args {
    #[structopt(name = "FILE")]
    pub file: PathBuf,
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt, Debug)]
pub(crate) enum Command {
    /// Encode a string as the supplied chunk type
    Encode {
        #[structopt(short = "t", parse(try_from_str))]
        chunk_type: ChunkType,
        #[structopt(name = "MESSAGE")]
        message: String,
    },
    /// Decode the supplied chunk type as a string
    Decode {
        #[structopt(short = "t", parse(try_from_str))]
        chunk_type: ChunkType,
    },
    /// Remove the supplied chunk type
    Remove {
        #[structopt(short = "t", parse(try_from_str))]
        chunk_type: ChunkType,
    },
    /// Print all chunks
    PrintAll,
}
