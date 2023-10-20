use std::fmt;
use std::fmt::Formatter;
use std::io::{BufReader, Read};
use crc::{Crc, CRC_32_ISO_HDLC};
use crate::chunk_type::ChunkType;

pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc_val: u32,
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} ", self.length, self.chunk_type)?;

        for &byte in &self.data {
            write!(f, "{} ", byte)?;
        }

        write!(f, "{} ", self.crc_val)
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = crate::Error;

    fn try_from(value: &[u8]) -> crate::Result<Self> {
        if value.len() >= 12 {
            let mut reader = BufReader::new(value);
            let mut buffer: [u8; 4] = [0, 0, 0, 0];

            reader.read_exact(& mut buffer)?;
            let chunk_length = u32::from_be_bytes(buffer);

            reader.read_exact(& mut buffer)?;
            let chunk_type = ChunkType::try_from(buffer)?;

            let mut buffer_vec = vec![0; chunk_length as usize];
            reader.read_exact(& mut buffer_vec)?;

            let data = buffer_vec;

            reader.read_exact(& mut buffer)?;
            let crc_val = u32::from_be_bytes(buffer);

            let tmp_crc = Chunk::get_crc(&chunk_type, &data);
            let res = Chunk {
                length: chunk_length,
                chunk_type,
                data,
                crc_val
            };
            if res.crc() == tmp_crc {
                Ok(res)
            } else {
                Err("Invalid CRC".into())
            }
        } else {
            Err("Invalid Chunk".into())
        }
    }
}

impl Chunk {
    pub fn get_crc(chunk_type: &ChunkType, data: &Vec<u8>) -> u32 {
        let crc_32 = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let mut digest = crc_32.digest();
        digest.update(&chunk_type.bytes());
        digest.update(&data);

        digest.finalize()
    }
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let tmp_crc = Chunk::get_crc(&chunk_type, &data);
        Chunk {
            length: data.len() as u32,
            chunk_type,
            data,
            crc_val: tmp_crc
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc_val
    }

    pub fn data_as_string(&self) -> crate::Result<String> {
        let res = String::from_utf8(self.data.clone());
        match res {
            Ok(s) => Ok(s),
            Err(e) => Err(e.into())
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length.to_be_bytes()
            .iter()
            .clone()
            .chain((self.chunk_type.bytes()).iter())
            .chain(self.data.iter())
            .chain(self.crc_val.to_be_bytes().iter())
            .cloned()
            .collect()
    }
}


#[allow(unused_variables)]
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
        let data = "This is where your secret message will be!".as_bytes().to_vec();
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

