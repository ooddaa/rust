// https://doc.rust-lang.org/book/ch05-01-defining-structs.html
struct BinaryTree<T, E> {
    left_node: Option<T>,
    right_node: Option<E>,
}

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    gay: bool,
}

fn build_vasya(age: u8, gay: bool) -> Person {
    Person {
        first_name: String::from("Vasya"),
        last_name: String::from("Pupkin"),
        age,
        gay,
    }
}

/* tuple structs */
/* unit-like structs -> tuple -> tuple structs -> structs in terms of # of namedness */

struct Point(u32, u32, u32);

fn main() {
    let mut vasya = build_vasya(33, true);
    println!("{:?}", vasya);

    vasya.gay = false;
    println!("{:?}", vasya);

    let petya = Person {
        first_name: String::from("Petya"),
        last_name: String::from("Vtoroy"),
        ..vasya
    };
    println!("{:?}", petya);

    let origin = Point(0, 0, 0);
    println!("{:?}", origin.1);
}
