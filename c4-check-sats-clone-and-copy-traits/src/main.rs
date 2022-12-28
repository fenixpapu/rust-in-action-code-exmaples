#[derive(Debug, Clone, Copy)] // Copy implies Clone, so we can use either trait later
struct CubeSat {
    id: u64,
}

#[derive(Debug, Clone, Copy)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    // sat_id.id += 1;
    StatusMessage::Ok
}

fn main() {
    println!("Hello, world!");
    let sat_a = CubeSat { id: 0 };

    let a_status = check_status(sat_a.clone()); // cloning each object is as easy as calling .clone()
    println!("a: {:?}", a_status.clone());

    let a_status = check_status(sat_a); // Copy works as expected
    println!("a: {:?}", a_status);

    let a_status = check_status(sat_a); // Copy works as expected
    println!("a: {:?}", a_status);
}
