use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, "Hello");
    map.insert(2, "World");

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}
