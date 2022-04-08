/* playing with enums */
/* https://www.youtube.com/watch?v=-E2qL4bLDKo&list=PLJbE2Yu2zumDF6BX6_RdPisRVHgzV02NW&index=6 */
#![allow(dead_code)]

#[derive(Debug)]
enum Direction {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}

enum Keys {
    UpArrow(String),
    DownArrow(String),
    LeftArrow(String),
    RightArrow(String),
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Direction {
    fn match_direction(&self) -> Keys {
        match *self {
            Direction::Up(_) => Keys::UpArrow(String::from("UpArrow pushed")),
            Direction::Down(_) => Keys::DownArrow(String::from("DownArrow pushed")),
            Direction::Left(_) => Keys::LeftArrow(String::from("LeftArrow pushed")),
            Direction::Right(_) => Keys::RightArrow(String::from("RightArrow pushed")),
        }
    }
}

impl Keys {
    fn print_key(&self, msg: String) -> String {
        println!("{}", msg);
        msg
    }

    fn destruct(&self) -> &String {
        match *self {
            Keys::UpArrow(ref s) => s,
            Keys::DownArrow(ref s) => s,
            Keys::LeftArrow(ref s) => s,
            Keys::RightArrow(ref s) => s,
        }
    }
}

enum Shape {
    Rectangle { width: u32, height: u32 },
    Square(u32),
    Circle(f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match *self {
            Shape::Rectangle { width, height } => (width * height) as f64,
            Shape::Square(side) => (side * side) as f64,
            Shape::Circle(r) => 3.14 * (r * r),
        }
    }
}

// enum Option<T> {
//     Some(T),
//     None,
// }

fn divide(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn main() {
    let dir = Direction::Down(Point { x: 1, y: 2 });
    let key = dir.match_direction();
    let x = key.destruct();

    println!("{}", x);

    let rec = Shape::Rectangle {
        width: 5,
        height: 3,
    };
    let sq = Shape::Square(10);
    let cir = Shape::Circle(4.0);

    println!("Rectangle area {}", rec.area());
    println!("Square area {}", sq.area());
    println!("Circle area {}", cir.area());

    // Option
    let res = divide(10.0, 2.1);
    match res {
        None => println!("Cannot divide by 0.0"),
        Some(d) => println!("Division == {:.4}", d),
    }

    let one: u8 = 0b0000_0001;
    let two: u16 = 0b0000_0000_0000_0010;
    println!("one: {}", one);
    println!("two is two: {}", two == 2);
}
