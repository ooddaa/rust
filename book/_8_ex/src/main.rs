// https://doc.rust-lang.org/book/ch08-03-hash-maps.html
use std::collections::HashMap;
use std::io;

// #![allow(dead_code)]
fn main() {
    // // avg
    // let ints = vec![1, 2];
    // let average = avg(ints);
    // println!("average {}", average);

    // // median
    // let med3 = median(vec![5, 2, 3, 1, 4]);
    // println!("med3 {:?}", med3);
    // let med25 = median(vec![2, 3, 1, 4]);
    // println!("med25 {:?}", med25);

    // // mode
    // let six = mode(vec![1, 6, 6, 6, 1, 3, 4, 5, 6]);
    // println!("six {:?}", six);

    // // pig-latin
    // println!("{:?}", pig_latin(String::from("first")));
    // println!("{:?}", pig_latin(String::from("apple")));

    // println!(
    //     "{:?}",
    //     split_sentence(String::from("Mary had a little lamb"))
    // );

    interface()
}

/* Given a list of integers, use a vector and return the mean (the average value),
median (when sorted, the value in the middle position),
and mode (the value that occurs most often; a hash map will be helpful here) of the list. */

fn avg(ints: Vec<usize>) -> f32 {
    let len = ints.len();
    match len {
        0 => return 0.0,
        _ => {
            let mut sum: usize = 0;
            for i in &ints {
                sum += i;
            }
            return sum as f32 / len as f32;
        }
    }
}

fn is_odd(int: &usize) -> bool {
    match int % 2 {
        0 => false,
        _ => true,
    }
}

// https://www.mathsisfun.com/median.html
fn median(ints: Vec<usize>) -> f32 {
    let len = ints.len();
    match len {
        0 => return 0.0,
        _ => {
            let mut to_sort = ints;
            to_sort.sort();
            match is_odd(&len) {
                // [1,2,3,4,5] len = 5 floor
                true => {
                    let i = len / 2;
                    let result = to_sort[i];
                    let resultf32 = result as f32;
                    // println!(
                    //     "odd len {}, i {:?}, result {}, resultf32 {}",
                    //     len, i, result, resultf32
                    // );
                    return resultf32;
                }
                false => {
                    // [1,2,3,4] 4 / 2 will get second value indexde~FEDFQazrtcvzev23 cfEX@cd3
                    let i = len / 2;
                    let first = to_sort[i - 1];
                    let second = to_sort[i];
                    let resultf32 = (first + second) as f32 / 2.0;

                    // println!(
                    //     "even len {}, first {}, second {}, resultf32 {}",
                    //     len, first, second, resultf32
                    // );
                    return resultf32;
                }
            }
            // return sum as f32 / len as f32;
        }
    }
}

fn mode(ints: Vec<usize>) -> usize {
    let mut map: HashMap<usize, usize> = HashMap::new();

    for val in ints {
        let count = map.entry(val).or_insert(0); // returns &mut V
        *count += 1;
    }
    println!("map {:?}", map);
    let mut m_freq: usize = 0;
    let mut count = 0; // omg I just put this into for in {} !!
    for (k, v) in map {
        if v > count {
            count = v;
            m_freq = k;
        }
    }

    return m_freq;
}

/*
Convert strings to pig latin. The first consonant of each word is moved to the end of the
word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have
“hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about
UTF-8 encoding!
*/

fn pig_latin(word: String) -> String {
    /* todo - need to check if the first letter/char is CAPITAL */
    if [97, 101, 105, 111, 117, 121].contains(&word.as_bytes()[0]) {
        return word + "-hay";
    } else {
        let first_letter = word[..1].to_string();

        let mut new_word = word[1..].to_string();
        new_word.push_str("-");
        new_word.push_str(&first_letter);
        new_word.push_str("ay");

        return new_word;
    }
}

fn split_sentence(sentence: String) -> Vec<String> {
    let words: Vec<&str> = sentence.split(' ').collect();
    let mut result: Vec<String> = vec![];
    for w in words {
        /* todo - need to clean up last word - it might have \n */
        result.push(String::from(w));
    }
    return result;
}

/*
Using a hash map and vectors, create a text interface to allow a user to add employee
names to a department in a company. For example, “Add Sally to Engineering” or
“Add Amir to Sales.” Then let the user retrieve a list of all people in a department
or all people in the company by department, sorted alphabetically.
*/

fn interface() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("Type Add (A) or Get (G)");
        // let t = String::from("test");
        // println!("test {}", t.as_str() == "test");
        let mut action = String::new();
        /* careful - any string supplied will have \n or 10 as_bytes as last char */
        io::stdin()
            .read_line(&mut action)
            .expect("Add(A) or Get(G)?");

        // action = action[..action.len() - 1].to_string();
        action = trim_input(action);

        println!("action.as_str() {}", &action.as_str());
        match action.as_str() {
            "A" | "a" | "Add" | "add" => {
                // println!("Adder {:?}", action.as_bytes());
                let (person, dep) = take_instruction("Person to Department");
                let ppl: &mut Vec<String> = map.entry(dep.to_string()).or_insert(Vec::new());
                ppl.push(person.to_string());
                println!("added {} to {}\n map is {:?}", person, dep, map);
                continue;
            }
            "G" | "g" | "Get" | "get" => {
                println!("Getter {:?}", action.as_bytes());

                println!("Name department {:?} or All", map.keys());
                let mut dep = String::new();
                io::stdin()
                    .read_line(&mut dep)
                    .expect("HR or Blowjoba4naya");
                dep = trim_input(dep);

                match dep.as_str() {
                    "All" => println!("{:?}", map),
                    _ => {
                        let ppl = &map[&dep];
                        println!("{}, {:?}", dep, ppl)
                    }
                }
            }
            _ => {
                println!("Typed {:?}", action.as_bytes());
                continue;
            }
        }
    }
}

fn take_instruction(msg: &str) -> (String, String) {
    println!("{}", msg);
    let mut instruction = String::new();
    io::stdin()
        .read_line(&mut instruction)
        .expect("Type: 'Sally to HR' or smth like that");
    instruction = trim_input(instruction);

    let split = split_sentence(instruction);
    if split.len() != 3 {
        panic!("wtf");
    }
    let person = String::from(split.get(0).unwrap()); //.get -> Option<&I::Output>
    let dep = String::from(split.get(2).unwrap()); //.get -> Option<&I::Output>
    return (person, dep);
}

fn trim_input(s: String) -> String {
    let bytes = &s.as_bytes();
    match bytes[bytes.len() - 1] {
        10 => return s[..s.len() - 1].to_string(),
        _ => return s,
    }
}
