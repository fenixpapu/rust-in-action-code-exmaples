fn main() {
    let result: f32 = 0.1 + 0.1;
    let desried: f32 = 0.2;
    let absolute_difference = (desried - result).abs();
    assert!(absolute_difference <= f32::EPSILON);
    println!("Hello, world!");
    // assert!(0.1 + 0.2 == 0.3);
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!(" 0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("       0.3: {:x}", (abc.2).to_bits());
    println!("");

    println!("xyz f(64)");
    println!(" 0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("       0.3: {:x}", (xyz.2).to_bits());
    println!("");

    assert!(abc.0 + abc.1 == abc.2); // run successfully
    assert!(xyz.0 + xyz.1 == xyz.2); // trigger a crash
}
