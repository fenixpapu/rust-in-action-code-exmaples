fn main() {
    println!("Hello, world!");
    let needle = 0o204;
    let haystack = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];

    for item in &haystack {
        // as in readme it like readonly
        if *item == needle {
            println!("{}", item);
        }
    }
}
