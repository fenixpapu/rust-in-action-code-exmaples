fn main() {
    println!("Hello, world!");
    let a = 10;
    let b = 20;
    let res = add_with_lifetimes(&a, &b);
    println!("{} + {} = {}", a, b, res);
    println!("a: {}, b: {} after call add_with_lifetime", a, b);
}

fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}
