// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
#![allow(dead_code)]
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
enum Message {
    Quit,
    Write(String),
    Send,
    Delete,
    ChangeColor(u8, u8, u8),
    Move { x: i32, y: i32 },
    Valid(bool),
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));

    // println!("{:?}", v4);
    // println!("{:?}", home);
    // println!("{:?}", loopback);
}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.htmlcd
#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        let maybe_some_string = Some(String::from("Hello, World!"));
        // `Option::map` takes self *by value*, consuming `maybe_some_string`
        let maybe_some_len = maybe_some_string.map(|s| s.len()); // maybe_some_string is not available here

        assert_eq!(maybe_some_len, Some(13));
    }
}
