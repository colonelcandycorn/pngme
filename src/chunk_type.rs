use std::str::FromStr;
use std::fmt;
use std::cmp;

#[derive(Debug)]
pub struct ChunkType {
	type_arr: [u8; 4],
}

impl ChunkType {
	pub fn bytes(&self) -> [u8; 4] {
		self.type_arr.clone()
	}

	pub fn is_valid(&self) -> bool {
		self.type_arr.iter().all(|byte| byte.is_ascii_alphabetic()) 
										&& self.is_reserved_bit_valid()
	}

	pub fn is_critical(&self) -> bool {
		self.type_arr.get(0).unwrap() & 32u8 == 0
	}

	pub fn is_public(&self) -> bool {
		self.type_arr.get(1).unwrap() & 32u8 == 0
	}


	pub fn is_reserved_bit_valid(&self) -> bool {
		self.type_arr.get(2).unwrap() & 32u8 == 0
	}

	pub fn is_safe_to_copy(&self) -> bool {
		self.type_arr.get(3).unwrap() & 32u8 != 0
	}
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = crate::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if value.iter().all(|byte| byte.is_ascii_alphabetic()) {
            Ok(ChunkType {type_arr: value} )
        } else {
            Err("Invalid ChunkType".into())
        }
    }
}

impl FromStr for ChunkType {
	type Err = crate::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let temp: &[u8] = s.as_bytes();
		if temp.len() == 4 {
			ChunkType::try_from([temp[0], temp[1], temp[2], temp[3]])
		} else {
			Err("Invalid length for ChunkType".into())
		}
	}
}

impl fmt::Display for ChunkType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let f_str = std::str::from_utf8(&self.type_arr).unwrap_or("Invalid UTF-8");
		write!(f, "{}", f_str)
	}
}

impl cmp::PartialEq for ChunkType {
	fn eq(&self, other: &Self) -> bool {
		self.type_arr == other.type_arr
	}
}

#[allow(unused_variables)]
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

