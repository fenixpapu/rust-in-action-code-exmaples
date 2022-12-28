#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

#[derive(Debug)]
struct Mailbox {
    message: Vec<Message>,
}

type Message = String;

struct GroundStation;

impl GroundStation {
    fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.message.push(msg);
    }
}

impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.message.pop()
    }
}

fn main() {
    println!("Hello, world!");
    let base = GroundStation {};
    let mut sat_a = CubeSat {
        id: 0,
        mailbox: Mailbox { message: vec![] },
    };
    println!("t0: {:?}", sat_a);

    base.send(&mut sat_a, Message::from("hello there!")); // We don't have a completely ergonomic way to create Message instance yet. Instead we'll take advantage of the String.from() method that converts &str to String (aka Message)
    println!("t1: {:?}", sat_a);

    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);

    println!("msg: {:?}", msg);
}
