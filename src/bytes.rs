use std::fs::File;
use std::io::{Cursor, Read, Seek};
use std::io::SeekFrom;
use byteordered::{ByteOrdered, Endianness};


#[derive(Debug)]
pub struct Bytes<R> {
    rdr: Option<ByteOrdered<R, Endianness>>,
    little_endian: bool
}


impl<R: Read + Seek> Bytes<R> {

    fn new() -> Bytes<R> {
        Bytes {
            rdr: None,
            little_endian: true
        }
    }

    fn set_reader(&mut self, rdr: R){
        self.rdr = Some(
            ByteOrdered::runtime(
                rdr,
                Endianness::le_iff(
                    self.little_endian
                )
            )
        )
    }

    pub fn seek(&mut self, pos: u64) {
        self.rdr.as_mut().unwrap().inner_mut().seek(
            SeekFrom::Start(pos)
        ).unwrap();
    }

    pub fn swap_order(&mut self) {
        self.little_endian = !self.little_endian;
        self.rdr = Some(
            self.rdr.take().unwrap().into_opposite()
        );
    }

    pub fn from_reader(rdr: R) -> Bytes<R> {
        let mut bytes = Bytes::new();
        bytes.set_reader(rdr);
        bytes
    }

    pub fn read_u16(&mut self) -> u16 {
        let num = self.rdr.as_mut().unwrap().read_u16().unwrap();
        num 
    }
  
    pub fn read_u32(&mut self) -> u32 {
        let num = self.rdr.as_mut().unwrap().read_u32().unwrap();
        num 
    }

    pub fn read_u64(&mut self) -> u64 {
        let num = self.rdr.as_mut().unwrap().read_u64().unwrap();
        num 
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_bytes_init() {
        let bytes = Bytes::<File> {
            rdr: None,
            little_endian: false
        };
        assert!(bytes.rdr.is_none());
        assert!(!bytes.little_endian);
    }

    #[test]
    fn test_bytes_from_reader() {
        let rdr = Cursor::new(vec![1, 0, 0, 0]);
        let bytes = Bytes::from_reader(rdr);
        assert!(bytes.rdr.is_some());
        assert!(bytes.little_endian);
    }
}
