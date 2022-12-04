use std::{fmt::Display, io::Bytes, ops::Range, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ChunkType([u8; 4]);

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.0
    }
    pub fn is_iend(&self) -> bool {
        self.to_string() == "IEND"
    }
    pub fn is_critical(&self) -> bool {
        self.0[0].is_ascii_uppercase()
    }
    pub fn is_public(&self) -> bool {
        self.0[1].is_ascii_uppercase()
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.0[2].is_ascii_uppercase()
    }
    pub fn is_safe_to_copy(&self) -> bool {
        self.0[3].is_ascii_lowercase()
    }
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0).to_string())
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = InvalidChunkType;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        let all_ascii_chars = bytes.iter().all(|byte| {
            let c = &(*byte as char);
            c.is_ascii_lowercase() || c.is_ascii_uppercase()
        });

        if all_ascii_chars {
            Ok(ChunkType(bytes))
        } else {
            Err(InvalidChunkType(
                String::from_utf8_lossy(&bytes).to_string(),
            ))
        }
    }
}

#[derive(Debug)]
pub struct InvalidChunkType(String);

impl Display for InvalidChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is not a valid chunk type", self.0)
    }
}

impl TryFrom<Vec<u8>> for ChunkType {
    type Error = InvalidChunkType;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let bytes: [u8; 4] = bytes
            .as_slice()
            .try_into()
            .map_err(|_| InvalidChunkType(String::from_utf8_lossy(&bytes).to_string()))?;

        ChunkType::try_from(bytes)
    }
}

impl FromStr for ChunkType {
    type Err = InvalidChunkType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: Vec<u8> = s.bytes().collect();
        ChunkType::try_from(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }
    //
    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
