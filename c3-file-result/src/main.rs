use rand::prelude::*;

fn one_in(denominator: u32) -> bool {
    // help function that triggers sporadic errors
    thread_rng().gen_ratio(1, denominator) // thread_rng() creates a thread-local random number generator; gen_ration(n,m) returns a Boolean value with an n/m probability
}

#[derive(Debug)]

struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        } // stylistic change to shorten the code block
    }
    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        // First appearance of Result<T, E> where T is an integer of type usize and E is a String. Using String provides arbitrary error messages
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length) // In this code, read() never fails, but we still wrap read_length in ok because we're returing Result
    }
}

fn open(f: File) -> Result<File, String> {
    if one_in(10_000) {
        // Once in 10,00 excutions, returns an error
        let err_mng = String::from("Permission denied!");
        return Err(err_mng);
    }
    Ok(f)
}

fn close(f: File) -> Result<File, String> {
    if one_in(100_000) {
        let err_mng = String::from("Interrupted by signal!");
        return Err(err_mng);
    }
    Ok(f)
}

fn main() {
    println!("Hello, world!");
    let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f4 = File::new_with_data("4.txt", &f4_data);

    let mut buffer: Vec<u8> = vec![];

    f4 = open(f4).unwrap(); // Unwraps T from Ok, leaving T
    let f4_length = f4.read(&mut buffer).unwrap();
    f4 = close(f4).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f4);
    println!("{} is {} bytes long", &f4.name, &f4_length);
    println!("{}", text);
}
