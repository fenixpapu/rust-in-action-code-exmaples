use std::thread; // bring multiple-threading in to local scope

fn main() {
    println!("Hello, world!");
    let mut data = 100;
    thread::spawn(|| {
        data = 500;
    }); // thread::spawn takes a closure as an argument
    thread::spawn(|| {
        data = 1000;
    });
    println!("{}", data);
}
