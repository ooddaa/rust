/* Here’s a small programming problem: write a function that takes
a string and returns the first word it finds in that string. */
#![allow(dead_code)]
fn main() {
    /* problem with this implementation is that I immediately overcomplicated things
    I defined a 'word' as
    a. collection of characters btw ranges 65-90 | 97-122
    AND
    b. ending with a safe ending (space, tab, newline, CR, !, ., ?)
    whereas a simpler satisfactory definition would be only (a)
    */
    fn first_word(s: &String) -> String {
        /* a word is all characters left of a whitespace\.\newline\tab
        any number of ascii symbols in ranges
        65-90 Capitals
        97-122 lower
        */
        let mut outside = true;
        let mut first_word: String = String::new(); // this will collect the first word

        for (_, letter) in s.chars().enumerate() {
            let n: i32 = letter as i32;
            match n {
                65..=90 | 97..=122 => {
                    if outside {
                        // if we were ouside the word, begin adding letters and change state
                        outside = false;
                        first_word.push(letter);
                        continue;
                    } else {
                        // if we are already inside the word, add letter to the word
                        first_word.push(letter);
                        continue;
                    }
                }
                _ => {
                    // if we have a non-letter
                    if outside {
                        continue;
                    } else {
                        // if we were inside and adding letters to the word
                        match n {
                            // and if we encountered a safe word-ending
                            // space, tab, newline, CR, !, ., ?
                            32 | 9 | 10 | 13 | 33 | 46 | 63 => {
                                return first_word;
                            }
                            _ => {
                                // need to go to the next space - too hard!
                                // to prevent "tab1e" returning "e"

                                // we will check final_word
                                first_word.push(letter);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        first_word
    }

    fn first_word_2(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }
        &s[..]
    }
    let s = String::from("lol and some");
    println!("{}", first_word(&s));
    // println!("{}", first_word_2(&s));

    let a = [1, 2, 3, 4];
    let a_slice = &a[..];
}
