use std::mem::transmute;

fn main() {
    println!("Hello, world!");
    let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let littel_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

    let a: i32 = unsafe { transmute(big_endian) };
    let b: i32 = unsafe { transmute(littel_endian) };

    println!("{} vs {}", a, b);
}
