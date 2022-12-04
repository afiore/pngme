mod args;
mod chunk;
mod chunk_type;
mod png;

use args::Args;
use std::io::{Seek, SeekFrom};
use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Write},
};
use structopt::StructOpt;

use crate::chunk::Chunk;
use crate::png::Png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Args::from_args();
    let file = File::options().read(true).write(true).open(args.file)?;

    let mut reader = BufReader::new(&file);
    let mut png =
        Png::try_from(&mut reader).map_err(|_| anyhow::format_err!("Invalid PNG file supplied"))?;

    match args.command {
        args::Command::Encode {
            chunk_type,
            message,
        } => {
            let chunk = Chunk::new(chunk_type, message.as_bytes().into());
            png.append_chunk(chunk);

            let mut writer = BufWriter::new(&file);
            writer.seek(SeekFrom::Start(0))?;
            writer.write_all(&png.as_bytes())?;
        }

        args::Command::Remove { chunk_type } => {
            png.remove_chunk(&chunk_type.to_string())
                .map_err(|_| anyhow::format_err!("Cannot find chunk type {}", chunk_type))?;

            let mut writer = BufWriter::new(&file);
            writer.seek(SeekFrom::Start(0))?;
            writer.write_all(&png.as_bytes())?;
        }

        args::Command::Decode { chunk_type } => {
            let chunk = png
                .chunk_by_type(&chunk_type.to_string())
                .ok_or_else(|| anyhow::format_err!("Cannot find chunk type {}", chunk_type))?;

            let message = chunk.data_as_string()?;
            println!("{}", message);
        }

        args::Command::PrintAll => {
            for chunk in png.chunks() {
                let message = chunk
                    .data_as_string()
                    .unwrap_or_else(|_| "0001010 BinGibbrish 000".to_owned());
                println!(
                    "chunk type: {}, length:{:>8}, crc:{:>12}| {}",
                    chunk.chunk_type(),
                    chunk.length(),
                    chunk.crc(),
                    message
                );
            }
        }
    }

    Ok(())
}
