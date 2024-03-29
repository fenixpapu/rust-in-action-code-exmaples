use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64, //Mhz
}

fn main() {
    println!("Hello, world!");
    let base: Rc<RefCell<GroundStation>> =
        Rc::new(RefCell::new(GroundStation { radio_freq: 87.65 }));
    println!("base: {:?}", base);

    {
        // Introduces a new scope where base can be mutably borrowed
        let mut base_2 = base.borrow_mut();
        base_2.radio_freq -= 12.34;
        println!("base: {:?}", base);
        println!("base_2: {:?}", base_2);
    }
    println!("base: {:?}", base);

    let mut base_3 = base.borrow_mut();
    base_3.radio_freq += 43.21;

    println!("base: {:?}", base);
    println!("base_3: {:?}", base_3);
}
