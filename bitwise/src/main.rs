// https://www.reidatcheson.com/hpc/architecture/performance/rust/c++/2019/10/19/measure-cache.html

fn main() {
    // println!("Hello, world!");
    let _n: u8 = 0b1111_1111;

    let a = 0b1000_0000 & _n;
    let b = (_n >> 7) & _n;
    let c = (_n << 7) & _n;
    println!("{}, {}, {}, {}", _n, a, b, c); // 256, 128, 1, 128

    let _m = 0b1000_0001;
    let d: u8 = _m << 1; // just want to make 2. if omit u8, it will be u16 == 256+2=258!
    println!("{}, {}", _m, d); // 129, 2

    let _o = 0.12345678901234567890_f32;
    println!("{}", _o); // 0.12345679 -> 8 digits -8 is max exponent

    let _p = 0.12345678901234567890_f64;
    println!("{}", _p); // 0.12345678901234568 -> -17

    let lol = 0x0000_000e; // hex
    println!("lol {}", lol); // 4
    let enough = "This should be enoug";
    println!("enough {}", enough.len());
}
