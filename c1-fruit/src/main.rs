fn main() {
    println!("Hello, world!");
    let fruit = vec![' ', ' ', ' '];
    let buffer_overflow = fruit[4];
    assert_eq!(buffer_overflow, '  ');
}
