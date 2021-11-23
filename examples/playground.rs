use const_index::{cindex, ConstGet};

fn main() {
    let bytes: Vec<u8> = (0..100).collect();

    let int = u64::from_le_bytes(*bytes.cindex(cindex!(..8)));

    println!("{:x}", int);
}
