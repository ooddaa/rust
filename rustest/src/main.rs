// use rand::prelude::*;

// fn main() {
//     let x:  u8 = random();
    
//     println!("sfdf! {}", x);
// }

// use ferris_says::say; // from the previous step
// use std::io::{stdout, BufWriter};

// fn main() {
//     let stdout  = stdout();
//     let message = String::from("Hello fellow Rustaceans!");
//     let width   = message.chars().count();

//     let mut writer = BufWriter::new(stdout.lock());
//     say(message.as_bytes(), width, &mut writer).unwrap();
// }

fn main() {
    let x: u8 = 254;

    fn lol(val: u8) -> u8 {
        return val + 1;
    }
    let res: u8 = lol(x);
    println!("keke X: {}", x); // valid coz x: u8
    println!("keke: {}", res);
}