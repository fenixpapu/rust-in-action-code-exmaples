#![allow(unused_variables)]

#[derive(Debug)]
struct File;

trait Read {
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

impl Read for File {
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        Ok(0) // A simple stub value that complies with the type signature required
    }
}

fn main() {
    println!("Hello, world!");
    let f = File {};
    let mut buffer = vec![];
    let n_bytes = f.read(&mut buffer).unwrap();
    println!("{} byte(s) read from {:?}", n_bytes, f);
}
