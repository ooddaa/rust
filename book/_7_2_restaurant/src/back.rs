#[derive(Debug)]
pub struct Breakfast {
    pub toast: String,      // customer decides
    drink: String,          // chef decides
    seasonal_fruit: String, // chef decides
}

impl Breakfast {
    pub fn serve_breakfast(toast: &str) -> Breakfast {
        let (drink, seasonal_fruit) = match toast {
            "wheat" => (String::from("coffee"), String::from("apple")),
            "rye" => (String::from("tea"), String::from("kiwi")),
            _ => (String::from("water"), String::from("banana")),
        };
        Breakfast {
            toast: String::from(toast),
            drink,
            seasonal_fruit,
        }
    }
}

pub enum Appetizer {
    Salad,
    Soup,
}
fn cancel_order() {}
fn fix_incorrect_order() {
    cancel_order();
    crate::front_of_house::serving::serve_order();
    super::front_of_house::serving::serve_order();

    let order1 = Appetizer::Salad;
    let order1 = Appetizer::Soup;
}
