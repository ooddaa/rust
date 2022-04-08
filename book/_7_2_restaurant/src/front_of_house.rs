pub mod hosting;

fn front_fun() {
    println!("front_fun here");
}

pub mod serving {
    pub fn serve_order() {}
    fn ask_if_everything_is_ok() {}
    fn take_payment() {}
}
