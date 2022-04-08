// https://doc.rust-lang.org/book/ch08-03-hash-maps.html
/* Like vectors, hash maps are homogeneous: all of the keys must
have the same type, and all of the values must have the same type. */
use std::collections::HashMap;
fn main() {
    let mut tool_albums = HashMap::new();

    let ae = String::from_utf8(vec![195, 134]).unwrap(); // c3 86 https://www.utf8-chartable.de/
                                                         // c3 == 12 3 == 1100_0011 == 128+64+0+0+0+0+2+1 = 195
                                                         // 86 == 8 6 == 1000_0100 = 128+0+0+0+0+6+0+0 = 134
    let aenima = ae + "nima";
    println!("{}", aenima);
    tool_albums.insert(String::from("10,000 Days"), 2006);
    tool_albums.insert(aenima, 2001);
    println!("tool_albums {:#?}", tool_albums);
    // tool_albums.insert(String::from("1"), true); // error

    // collect
    // let a: Vec<String> = vec!["a".into(), "b".into(), "c".into()]; // either
    let a = vec!["a", "b", "c"]; // either
    let b = vec![1, 2, 3];
    let mut obj: HashMap<_, u8> = a.into_iter().zip(b.into_iter()).collect();
    println!("obj {:#?}", obj);
    println!("obj[a] {:#?}", obj["a"]); // 1
                                        // println!("obj[d] {:#?}", obj["d"]); // panics
    println!("obj.get(a) {:#?}", obj.get("a")); // works coz .get(&str) -> Option<&V>. Some(&1)
    println!("obj.get(x) {:#?}", obj.get("x")); // None

    // For types that implement the Copy trait, like i32, the values are copied into the hash map.
    // For owned values like String, the values will be moved and the hash map will be the owner of those values
    // println!("a[1] {}", a[1]) // a[1] was borrowed by hashmap

    // If we insert references to values into the hash map, the values won’t be moved into the hash map.
    let grudge = "Grudge".to_string();
    obj.insert(&grudge, 4);
    println!("obj {:#?}, grudge: {}", obj, grudge);

    // ENTRY
    tool_albums.entry("10,000 Days".to_string()).or_insert(666);
    tool_albums.entry("Lateralus".to_string()).or_insert(2001);
    println!("iterate over kv tool_albums:");
    for (k, v) in &tool_albums {
        println!("{}: {}", k, v);
    }

    // update depending on old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // returns &mut V
        *count += 1;
    }
    println!("{:?}", map);
}
