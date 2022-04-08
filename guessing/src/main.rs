use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

fn main() {
    println!("Enter your number");

    let r = thread_rng().gen_range(1, 101);
    // println!("random number is {}\n", r);
    loop {
        let mut guess = String::new(); // allocate some memory for a string. guess is 'owner' of value Sring::new()
                                       /*
                                        The :: syntax in the ::new line indicates that new is an associated
                                        function of the String type. An associated function is implemented
                                        on a type, in this case String, rather than on a particular instance
                                        of a String. Some languages call this a static method.
                                       */

        io::stdin() // The stdin function returns an instance of std::io::Stdin
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed {}\n", guess);

        // let guess: i32 = guess.trim().parse().expect("Type in a number!");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&r) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                // return println!("Bingo!");
                println!("Bingo!");
                break;
            }
        };
    }
}

// fn r#fn() {}
