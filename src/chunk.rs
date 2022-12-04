use std::{fmt::Display, str::Utf8Error};

use crc::{Algorithm, Crc};

use crate::chunk_type::{self, ChunkType};

#[derive(Clone)]
pub(crate) struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Into<(ChunkType, Chunk)> for Chunk {
    fn into(self) -> (ChunkType, Chunk) {
        (self.chunk_type.clone(), self)
    }
}

impl Chunk {
    pub fn crc_checksum(chunk_type: &ChunkType, data: &Vec<u8>) -> u32 {
        let mut all_data: Vec<u8> = chunk_type.bytes().to_vec();
        all_data.extend(data.clone());

        let crc: Crc<u32> = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        crc.checksum(all_data.as_slice())
    }

    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let crc = Chunk::crc_checksum(&chunk_type, &data);
        Chunk {
            chunk_type,
            data,
            crc,
        }
    }
    pub fn length(&self) -> usize {
        self.data.len()
    }
    pub fn crc(&self) -> u32 {
        self.crc
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data_as_string(&self) -> Result<String, Utf8Error> {
        let s = std::str::from_utf8(&self.data)?;
        Ok(s.to_owned())
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let length = self.data.len() as u32;

        bytes.extend(length.to_be_bytes());
        bytes.extend(self.chunk_type.bytes());
        bytes.extend(self.data.clone());
        bytes.extend(self.crc.to_be_bytes());

        bytes
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = String::from_utf8_lossy(&self.data);
        write!(f, "{}", data)
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ();

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 12 {
            Err(())
        } else {
            let length: [u8; 4] = bytes[0..4].try_into().map_err(|_| ())?;
            let length = u32::from_be_bytes(length);

            let chunk_type: [u8; 4] = bytes[4..8].try_into().map_err(|_| ())?;
            let chunk_type = ChunkType::try_from(chunk_type).map_err(|_| ())?;

            let data: Vec<u8> = bytes
                .iter()
                .skip(8)
                .take(length as usize)
                .copied()
                .collect();

            let crc_bytes: [u8; 4] = bytes
                .iter()
                .rev()
                .take(4)
                .copied()
                .collect::<Vec<u8>>()
                .try_into()
                .map_err(|_| ())?;

            let crc = u32::from_le_bytes(crc_bytes);
            let computed_crc = Chunk::crc_checksum(&chunk_type, &data);

            if data.len() == length as usize && computed_crc == crc {
                Ok(Chunk::new(chunk_type, data))
            } else {
                Err(())
            }
        }
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

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
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
    fn test_roundtrip() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let chunk = Chunk::new(chunk_type, message_bytes.into());
        let crc: u32 = 2882656334;

        assert_eq!(crc, chunk.crc())
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
