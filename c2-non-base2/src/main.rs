fn main() {
    println!("Hello, world!");
    let three = 0b11; // 0b is base2
    let thirty = 0o36; // 0o is base8
    let three_hundered = 0x12C; // 0x is base16

    println!("base 10: {}, {}, {}", three, thirty, three_hundered);
    println!("base 2: {:b}, {:b}, {:b}", three, thirty, three_hundered);
    println!("base 8: {:o}, {:o}, {:o}", three, thirty, three_hundered);
    println!("base 16: {:x}, {:x}, {:x}", three, thirty, three_hundered);
}
