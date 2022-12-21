fn main() {
    println!("Hello, world!");
    let a = 10; // types can be inferred by compiler...
    let b: i32 = 20; // or declare by programmer when createing variables
    let c = 30i32; // Numeric types can include a type annotation in their literal form.
    let d = 30_i32; // Numberics can include underscores, which are intended to increase readability and have no function impact
    let e = add(add(a, b), add(c, d));
    println!("( a + b ) + ( c + d ) = {}", e);
}

fn add(i: i32, j: i32) -> i32 {
    // type declaration are required when defineing functions
    i + j // Functions return the last expression's result so that return is not required.
          // Dont push ';' at line 13. This changes the semantics, returning () (unit) rather than i32.
}
