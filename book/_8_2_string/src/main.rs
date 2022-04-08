// https://doc.rust-lang.org/book/ch08-02-strings.html
// https://doc.rust-lang.org/std/string/struct.String.html
/*
A String is made up of three components:
a pointer to some bytes, as_ptr()
a length, and len()
a capacity capacity()

*/
fn main() {
    let s1 = String::from("a ");
    let s2: String = "b".into();
    let s3 = s1 + &s2; // s1 moved, compiler deref coerces s2: String -> &s2: &String -> &s2[..]: &str
                       // println!("{}", s1); // wont work
    let mut s3 = s3 + " c";
    println!("s3: {}", &s3); // "a b c"
                             // let mut s3_1 = s3.push(' '); // it returns ()
    s3.push(' ');
    s3.push('d');
    s3.push_str(" e");
    println!("s3: {}", &s3); // "a b c d"

    // pointer len capacity
    println!(
        "s3: ptr {:?}, len {}, cap {}",
        s3.as_ptr(),
        s3.len(),
        s3.capacity()
    );
    let tic = "tic".to_string();
    let tac: String = "tac".into();
    let toe = String::from("toe");
    let s4 = format!("{}-{}-{}", tic, tac, toe);
    println!("s4: {}", s4); //doesn’t take ownership of any of its parameters.
    println!("{}-{}-{}", tic, tac, toe);

    // println!("{}", tic[1]); // no idexing into String.

    println!("tic as bytes {:?}", tic.into_bytes()); // [116, 105, 99]
    let tid = String::from_utf8(vec![116, 105, 100]); // returns Ok("tid")
    println!("tid {}", tid.unwrap());

    // https://unicode.org/emoji/charts/full-emoji-list.html#1f601

    // let hearty_eyes = String::from_utf8(vec![]); // U+1F60D
    // println!("tid {}", hearty_eyes.unwrap());
    let euro = String::from_utf8(vec![226, 130, 172]); // U+20AC -> E2 82 AC -> https://en.wikipedia.org/wiki/UTF-8
    println!("euro {}", euro.unwrap());
    println!("euro € is {:?}", "€".to_string().as_bytes()); // [226, 130, 172]

    let broken_heart = String::from_utf8(vec![240, 159, 146, 148]); //f09f9294 U+1F494
    println!("broken_heart {}", broken_heart.unwrap());

    //
    let hindi = String::from_utf8(vec![
        224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135,
    ])
    .unwrap();
    println!("hindi {}, len: {}", &hindi, hindi.len());

    println!("hindi chars");
    for c in hindi.chars() {
        println!("{}", c);
    }

    println!("hindi bytes");
    for c in hindi.bytes() {
        println!("{}", c);
    }

    let mut s_with_cap = String::with_capacity(1);

    for _ in 0..6 {
        s_with_cap.push_str("hello");
        println!("s_with_cap cap: {}", s_with_cap.capacity()) // 5 10 20 20 40 40
    }

    let mut tool = String::from("tool");
    tool.remove(1); // o
    tool.remove(1); // 0
    println!("tl: {}", tool);

    let mut dany_carey = String::from("Dany Carey");
    dany_carey.pop();
    dany_carey.pop();
    println!("Dany Car: {}", dany_carey);
    dany_carey.truncate(4); // Dany
    println!("Dany: {}", dany_carey);

    //retain
    let mut adam_jones = String::from("A_d_a_m== J_ones");
    adam_jones.retain(|c| c != '_' && c != '=');
    println!("Adam Jones: {}", adam_jones);

    //insert
    let mut keenan = String::from("eenan");
    keenan.insert(0, 'K');
    println!("Keenan: {}", keenan);

    //insert_str
    keenan.insert_str(keenan.len(), " James");
    println!("Keenan James: {}", keenan);
}
