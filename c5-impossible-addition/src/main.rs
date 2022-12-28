#[allow(arithmetic_overflow)] //Required declaration. The Rust compiler can detect this obvious overflow situation.
fn main() {
    println!("Hello, world!");
    let (a, b) = (200, 200);
    let c: u8 = a + b; // Without the type declaration. Rust won't assume that you're trying to create an impossible situation.
    println!("200 + 200= {}", c);
}
