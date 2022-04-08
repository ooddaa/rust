enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

#[derive(Debug)]
enum USState {
    NewYork(String),
    California(String),
    Nevada(String),
}

fn main() {
    // println!(
    //     "{}",
    //     coin_in_cents(Coin::Quarter(USState::Nevada(String::from("Nevada"))))
    // )
    println!("{:?}", deal_with_option(Some(1)));
    println!("{:?}", deal_with_option(None));

    if let Some(1) = Some(1) {
        Some(1 + 1);
    }
    let coin = Coin::Quarter(USState::Nevada(String::from("Nevada")));
    if let Coin::Quarter(state) = coin {
        println!("Woa, we've got a {:?}", state);
    } else {
        println!("Wasn't a quarter");
    }
}

fn coin_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => return 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("25 from {:?}", state);
            25
        }
    }
}

fn deal_with_option(val: Option<i32>) -> Option<i32> {
    match val {
        Some(v) => Some(v + 1),
        None => None,
    }
}
