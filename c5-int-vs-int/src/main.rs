fn main() {
    println!("Hello, world!");
    let a: u16 = 50115;
    let b: i16 = -15421;

    println!("a: {:016b} {}", a, a); // these two values have the same bit pattern but different types
    println!("b: {:016b} {}", b, b);
}
