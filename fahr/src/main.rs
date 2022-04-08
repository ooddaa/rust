use std::io;

fn ensure_input_is_num(print_msg: &str, exp_msg: &str) -> f32 {
    let num: f32 = loop {
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

fn main() {
    let syst: f32 = ensure_input_is_num(
        "Do you want to convert (1) Fahrenheit or (2) Celsium?",
        "Expected 1 or 2",
    );

    match (syst) as i32 {
        1 => {
            let fahr: f32 = ensure_input_is_num(
                "Type in a Fahrenheit number",
                "Expected a Fahrenheit number",
            );
            let cels: f32 = 5. * (fahr - 32.) / 9.;
            println!("Fahr\tCels");
            println!("{}\t{}", fahr, cels);
        }
        2 => {
            let cels: f32 =
                ensure_input_is_num("Type in a Celsium number", "Expected a Celsium number");

            let fahr: f32 = cels / 5.0 * 9.0 + 32.0;
            println!("Cels\tFahr");
            println!("{}\t{}", cels, fahr);
        }
        _ => {
            println!("exiting");
            return;
        }
    }
}

/*

5 * (x - 32) / 9 == 9 * (x / 5) + 32

*/
