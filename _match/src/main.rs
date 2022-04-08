fn main() {
    let capital_range = 65..=90;
    let x = 32;

    match x {
        // capital_range => println!("cap matched"), // this wont work!!
        65..=90 => println!("cap matched"),
        _ => println!("_ matched"),
    }
}
