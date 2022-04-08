// https://doc.rust-lang.org/book/ch08-01-vectors.html
// vector puts all the values next to each other in memory
// https://doc.rust-lang.org/std/vec/index.html
// Vectors have O(1) indexing, amortized O(1) push (to the end) and O(1) pop (from the end).
// Vectors ensure they never allocate more than isize::MAX bytes.
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);

    match v.get(2) {
        Some(e) => println!("v third: {}", e), // &e here
        None => println!("v: none"),
    }

    let v1 = vec![1, 2, 3];
    let third: &i32 = &v1[2];
    println!("v1 third: {}", third);

    let mut v2 = vec![1, 2, 3, 4];
    let second = &v2[1]; // read-only immutable reference. need to drop before mutating v2
                         // v2.push(5); // mutable borrow occurs here, would be fine if I didnt use 'second' later
    println!("v2 second: {}", second);
    v2.pop();
    println!("v2: {:?}", v2); // [1, 2, 3]
    v2[1] = v2[1] + 10;
    println!("v2: {:?}", v2); // [1, 12, 3]

    println!("isize::MAX: {}", isize::MAX); // 9,223,372,036,854,775,807

    let ten_zeros = vec![0; 10];
    println!("ten_zeros: {:?}", ten_zeros);

    // ITERATE
    for i in &v2 {
        // just have a look
        println!("i: {}", i);
    }

    for i in &mut v2 {
        // have a look AND be able to write
        // is * what gives us the VALUE to which pointer points? ie 1, 12, 3?
        // whereas i (without *) == a &mut i32?
        println!("{:?}", i);
        *i = *i + 1;
    }
    println!("v2: {:?}", v2); // [2, 13, 4]

    // when we need diff types in vector
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f32),
        Text(String),
    }
    let ss = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(12.12),
        SpreadsheetCell::Text(String::from("Tool is great!")),
    ];
    println!("ss: {:?}", ss);

    // sort
    let mut to_sort = vec![5, 2, 3, 1, 4];
    to_sort.sort();
    // println!("to_sort {:?}", to_sort);
}
