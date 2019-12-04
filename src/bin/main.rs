use rig::bigwig::BigWig;
use rig::bytes::Bytes;
use std::io::{Cursor};
use std::fs::File;


fn main() {
    let mut bw = BigWig::from_path(
        "/Users/keenan/Desktop/ENCFF457TKX.bigWig"
    ).open().unwrap();
    println!("{:?}", bw);
    println!("{}", bw.is_bigwig());
    println!("{}", bw.validate_and_set_endianness());
    println!("{}", bw.is_bigwig());
    println!("{}", bw.read_magic_number().unwrap());
    println!("{}", bw.read_magic_number().unwrap());
    println!("{}", bw.read_magic_number().unwrap());
    let rdr = Cursor::new(vec![2, 1, 0, 0]);
    let mut bw = BigWig::from_reader(
        rdr
    );
    println!("{}", bw.is_bigwig());
    let mut f = File::open("/Users/keenan/Desktop/ENCFF457TKX.bigWig").unwrap();
    let mut b = Bytes::from_reader(f);
    let num = b.read_u32();
    println!("{:?}", num);
    b.seek(0);
    let num = b.read_u32();
    println!("{:?}", num);
    b.seek(0);
    b.swap_order();
    let num = b.read_u32();
    println!("{:?}", num);
    b.seek(0);
    let num = b.read_u32();
    println!("{:?}", num);
    let num = b.read_u32();
    println!("{:?}", num);
    b.swap_order();
    let num = b.read_u32();
    println!("{:?}", num);
    let num = b.read_u32();
    println!("{:?}", num);
    b.seek(0);
    let num = b.read_u32();
    println!("{:?}", num);
    let num = b.read_u16();
    b.seek(0);
    println!("{:?}", num);
    let num = b.read_u64();
    println!("{:?}", num);
    let rdr = Cursor::new(vec![2, 1, 0, 0]);
    let mut b = Bytes::from_reader(
        rdr
    );
    let num = b.read_u32();
    println!("{:?}", num);
}
