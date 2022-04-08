/* Generate the nth Fibonacci number. */
/* https://www.omnicalculator.com/math/fibonacci */
use std::io;

fn ensure_input_is_num(print_msg: &str, exp_msg: &str) -> i32 {
    let num: i32 = loop {
        let mut num = String::new();
        println!("{}", print_msg);
        io::stdin().read_line(&mut num).expect(&exp_msg);
        match num.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        }
    };
    return num;
}

fn fib(num: i32) -> f64 {
    let gr: f64 = (1. + 5f64.sqrt()) / 2.;
    let gamma: f64 = 1f64 - gr;
    let res: f64 = (gr.powf(num as f64) - gamma.powf(num as f64)) / 5f64.sqrt();
    return res;
}

fn main() {
    let num: i32 = ensure_input_is_num("Enter number", "oooops");
    let result: f64 = fib(num);
    println!("Result: {}", result);
}
