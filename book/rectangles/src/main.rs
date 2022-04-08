// https://doc.rust-lang.org/book/ch05-02-example-structs.html

fn main() {
    let w1: u32 = 30;
    let h1: u32 = 50;
    let tup = (w1, h1);

    println!("Area 1 is {}", area(w1, h1));
    println!("Area 2 is {}", area1(tup));

    let rect = Rectangle {
        width: w1,
        height: h1,
    };
    println!("Area 3 is {}", rect.area());
    println!("Area 4 is {}", area2(&rect));
    println!("{:#?}", rect);

    let rect2 = Rectangle {
        width: w1 + 1,
        height: h1 - 1,
    };
    println!("can_hold {}", rect.can_hold(&rect2));

    let sq = Rectangle::square(3);
    println!("{:#?}", sq);
}

fn area(w: u32, h: u32) -> u32 {
    w * h
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
