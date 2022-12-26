struct Hostname(String);

fn connect(host: Hostname) {
    println!("connected to {}", host.0);
}

fn main() {
    println!("Hello, world!");
    let ordinary_string = String::from("localhost");
    let host = Hostname(ordinary_string.clone());

    connect(ordinary_string);
}
