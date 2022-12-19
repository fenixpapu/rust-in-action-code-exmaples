#[derive(Debug)] // Allow the println! marco to print Cereals num

enum Cereal {
    Barley,
    Millet,
    Rice,
    Rye,
    Spelt,
    Wheat,
}

fn main() {
    println!("Hello, world!");
    let mut grains: Vec<Cereal> = vec![];
    grains.push(Cereal::Rye);
    drop(grains);
    println!("{:?}", grains);
}
