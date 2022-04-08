use std::cmp::Ordering;

fn main() {
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS {}", MAX_POINTS);

    let spaces = "   ".len();
    println!("spaces.len {}", spaces);
    let word = "lol";
    match "lol".cmp(&word) {
        Ordering::Less => println!("Less"),
        Ordering::Greater => println!("Greater"),
        Ordering::Equal => println!("Ok"),
    }

    match "lol" {
        "lol" => println!("lol ok"),
        _ => println!("lol not ok"),
    }

    let num = 5_7_usize; // https://doc.rust-lang.org/book/ch03-02-data-types.html
    println!("num is {}", num); // 57
}
