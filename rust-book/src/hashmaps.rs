use std::collections::HashMap;

/// In RUST hashmap owns an inserted element!

fn main() {
    let mut hashmap = HashMap::new();
    hashmap.insert("Red".to_string(), 0);
    hashmap.insert("Green".to_string(), 127);
    hashmap.insert("Blue".to_string(), 256);

    let key: String = "Blue".to_string();
    let key_non_existing: String = "Blue-2".to_string();

    println!("hashmap[\"{}\"] = {}", key, hashmap.get(&key).unwrap());
    println!("hashmap[\"{}\"] = {}", key_non_existing, hashmap.get(&key_non_existing).unwrap_or(&-1));

    for (key, value) in &hashmap {
        println!("{}: {}", key, value);
    }

    println!("{}", hashmap.entry(String::from("Yellow")).or_insert(20));
    println!("{}", hashmap.entry(String::from("Blue")).or_insert(13));


    println!("{:?}", hashmap);
}
