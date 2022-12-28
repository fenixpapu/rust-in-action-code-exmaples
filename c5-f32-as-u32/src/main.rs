fn main() {
    println!("Hello, world!");
    let a: f32 = 42.42;
    let frankentype: u32 = unsafe {
        std::mem::transmute(a) // No semicolon here. We want the result of this expression to feed into the outer scope.
    };

    println!("{}", frankentype); // View the bits of a 42.42_f32 value as a decimal integer
    println!("{:032b}", frankentype); //{:032b} means to format as binary via the std::fmt:Binary with 32 zeroes padded on the left

    let b: f32 = unsafe { std::mem::transmute(frankentype) };

    println!("{}", b); // Confirms that the operation is symmetrical
    assert_eq!(a, b);
}
