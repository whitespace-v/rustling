use std::collections::HashMap;

fn main() {
    // hash maps -> heap
    let mut a = HashMap::new();
    a.insert(String::from("demo"), 42);
    a.insert(String::from("key"), 10);

    // let value = a.get(&String::from("key")); // option
    // let value = a.get(&String::from("key")).unwrap();
    // let value = a.get(&String::from("key")).copied().unwrap_or(1); // with default value or: a.get -> & so we need &1 -> so we can use like it or like ->
    let value = a.get(&String::from("key")).unwrap_or(&1);

    a.entry(String::from("demo")).or_insert(50); // if exists -> 42, if not exist -> 50

    for (k, v) in &a {
        println!("{k}, {v}")
    }
}
