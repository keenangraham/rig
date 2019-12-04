use std::error::Error;
use std::fs::File;
use std::io::{Cursor, Read, Seek};
use std::path::{PathBuf};
use crate::bytes::{Bytes};

const BIGWIG_MAGIC_LE: u32 = 0x888FFC26;


#[derive(Debug)]
struct BigWigHeader {
    magic: u32,
    version: u16,
    zoom_levels: u16,
    chrom_tree_offset: u64,
    unzoomed_data_offset: u64,
    unzoomed_index_offset: u64,
    field_count: u16,
    defined_field_count: u16,
    as_offset: u64,
    total_summary_offset: u64,
    uncompressed_buffer_size: u32,
    extension_offset: u64
}


#[derive(Debug)]
pub struct BigWig<R: Read + Seek> {
    path: Option<PathBuf>,
    header: Option<BigWigHeader>,
    bytes: Option<Bytes<R>>,
}


impl BigWig<File> {
    
    pub fn open(mut self) -> Result<BigWig<File>, Box<dyn Error>> {
        let path = self.path.as_ref().unwrap();
        let rdr = File::open(&path)?;
        self._set_bytes_from_reader(rdr);
        Ok(self)
    }

}


impl<R: Read + Seek> BigWig<R> {

    pub fn from_path(path: &str) -> BigWig<R> {
        BigWig {
            path: Some(PathBuf::from(path)),
            bytes: None,
            header: None,
        }
    }

    pub fn new() -> BigWig<R> {
        BigWig {
            path: None,
            bytes: None,
            header: None
        }
    }

    fn _set_bytes_from_reader(&mut self, rdr: R) {
        let bytes = Bytes::from_reader(rdr); 
        self.bytes = Some(bytes);
    }

    pub fn from_reader(rdr: R) -> BigWig<R> {
        let mut bw = BigWig::new();
        bw._set_bytes_from_reader(rdr);
        bw
    }

    pub fn read_magic_number(&mut self) -> Result<u32, Box<dyn Error>> {
        let bytes = self.bytes.as_mut().unwrap();
        bytes.seek(0);
        let num = bytes.read_u32();
        Ok(num)
    }

    pub fn is_bigwig(&mut self) -> bool {
        let magic_number = self.read_magic_number().unwrap();
        magic_number == BIGWIG_MAGIC_LE
    }

    pub fn validate_and_set_endianness(&mut self) -> bool {
        if !self.is_bigwig() {
            self.bytes.as_mut().unwrap().swap_order();
        }
        self.is_bigwig()
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_big_wig_init() {
        let bw = BigWig::<File> {
            header: None,
            path: Some(PathBuf::from("abc")),
            bytes: None
        };
        assert_eq!(bw.path.unwrap().to_str().unwrap(), "abc");
    }

    
    #[test]
    fn test_big_wig_from_path() {
        let bw = BigWig::<File>::from_path("abc");
        assert_eq!(bw.path.unwrap().to_str().unwrap(), "abc");
        assert!(bw.header.is_none());
    }


    #[test]
    fn test_big_wig_from_reader() {
        let rdr = Cursor::new(vec![1, 0, 0, 0]);
        let mut bw = BigWig::from_reader(rdr);
        let num = bw.read_magic_number().unwrap();
        assert_eq!(num, 1);
        let rdr = Cursor::new(vec![0, 1, 0, 0]);
        let mut bw = BigWig::from_reader(rdr);
        let num = bw.read_magic_number().unwrap();
        assert_eq!(num, 256);
    }
}
