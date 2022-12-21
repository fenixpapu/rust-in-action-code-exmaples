fn main() {
    println!("Hello, world!");
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    let addition = twenty + twenty_one + twenty_two;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );

    let one_million: i64 = 1_000_000; // underscores increase readability and are ignored by compiler
    println!("{}", one_million.pow(2));

    let forty_twos = [
        // create an array of numbers which must all be the same type
        42.0, 42f32, 42.0_f32,
    ];

    println!("{:02}", forty_twos[0]);
}
