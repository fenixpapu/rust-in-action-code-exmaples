use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    println!("Hello, world!");
    let a = 10;
    let b = Box::new(20); // integer on the heap also know as a boxed integer
    let c = Rc::new(Box::new(30)); // boxed integer wrapped within an atomic reference counter
    let d = Arc::new(Mutex::new(40)); // boxed integer wrapped within an atomic reference counter and protected by a mutual exclusion lock
    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d);
}
