use restaurant::back::Breakfast;
use restaurant::Appetizer;

fn main() {
    let b = Breakfast::serve_breakfast("rye");
    println!("{:?}", b);

    use std::collections::HashMap;
    let mut nums = HashMap::new();
    nums.insert("first", 1);
    nums.insert("tenth", 10);

    println!("{:?}", nums); // {"first": 1, "tenth": 10}
    println!("{:?}", nums["first"]); // 1

    // use std::fmt;
    // use std::io;
    // use std::io::Result as IoResult;
    // use std::{cmp::Ordering, fmt, io, io::Result as IoResult};
    use std::io::{self, Result as IoResult};
    use std::{cmp::Ordering, fmt};

    fn f() -> io::Result<String> {
        Ok(String::from("alright!"))
    }
    fn f1() -> IoResult<String> {
        Ok(String::from("still ok"))
    }
    fn f2() -> fmt::Result {
        Ok(())
    }

    let salad = Appetizer::Salad;

    println!("{}", stringify!(nums)); // prints "nums"

    println!("MIN i32 {}", i32::MIN);
    println!("MAX i32 {:#?}", i32::MAX);
    // unimplemented! {}

    let krovostok = "Я плотник по сухожилиям...";
    let kp: *const u8 = krovostok.as_ptr(); // u8 max is 255 !
    println!("krovostok.as_ptr {:?}", kp); // 0x 1 0 a 1 d b 6 8 0  0x10a1db680
    println!("krovostok.len {}", krovostok.len());
}
