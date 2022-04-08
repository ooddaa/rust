/* Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”
taking advantage of the repetition in the song. */
// #![allow(dead_code)]

fn main() {
    // https://stackoverflow.com/questions/30154541/how-do-i-concatenate-strings
    let on_the: String = String::from("On the");
    let day_of: String = String::from("day of Christmas\nMy true love gave to me");
    let months: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    fn gifts(month: usize) -> String {
        let podarki: [&str; 12] = [
            "12 drummers drumming",
            "Eleven pipers piping",
            "Ten lords a leaping",
            "Nine ladies dancing",
            "Eight maids a milking",
            "Seven swans a swimming",
            "Six geese a laying",
            "Five gold rings, badam-pam-pam",
            "Four calling birds",
            "Three French hens",
            "Two turtle doves",
            "And a partridge in a pear tree",
        ];
        let podarki_len = podarki.len();
        match month {
            1 => return String::from("A partridge in a pear tree"),
            2..=12 => {
                return podarki[(podarki_len - month)..podarki_len].join("\n");
            }
            _ => return String::from("lol"),
        }
    }
    for i in 0..11 {
        println!("{} {} {}\n{}\n", on_the, months[i], day_of, gifts(i + 1))
    }
}

/*
On the first day of Christmas
My true love gave to me
A partridge in a pear tree

On the second day of Christmas
My true love gave to me
Two turtle doves
And a partridge in a pear tree

On the third day of Christmas
My true love gave to me
Three French hens
Two turtle doves
And a partridge in a pear tree

On the fourth day of Christmas
My true love gave to me
Four calling birds
Three French hens
Two turtle doves
And a partridge in a pear tree

On the fifth day of Christmas
My true love gave to me
Five gold rings
Four calling birds
Three French hens
Two turtle doves
And a partridge in a pear tree

On the sixth day of Christmas
My true love gave to me
Six geese a laying
Five gold rings
Four calling birds
Three French hens
Two turtle doves
And a partridge in a pear tree

On the seventh day of Christmas
My true love gave to me
Seven swans a swimming
Six geese a laying
Five gold rings
Four calling birds
Three French hens
Two turtle doves
And a partridge in a pear tree

On the eighth day of Christmas
My true love gave to me
Eight maids a milking
Seven swans a swimming
Six geese a laying
Five gold rings
Four calling birds
Three French hens
Two turtle doves
And a partridge in a pear tree

On the ninth day of Christmas
Me me me me me me
Nine ladies dancing
Eight maids a milking
Seven swans a swimming
Six geese a laying
Five gold rings, badam-pam-pam
Four calling birds
Three French hens
Two turtle doves
And a partridge in a pear tree

On the tenth day of Christmas
My true love gave to me
Ten lords a leaping
Nine ladies dancing
Eight maids a milking
Seven swans a swimming
Six geese a laying
Five gold rings, badam-pam-pam
Four calling birds
Three French hens
Two turtle doves
And a partridge in a pear tree

On the eleventh day of Christmas
My true love gave to me
Eleven pipers piping
Ten lords a leaping
Nine ladies dancing
Eight maids a milking
Seven swans a swimming
Six geese a laying
Five gold rings, badam-pam-pam
Four calling birds
Three French hens
Two turtle doves
And a partridge in a pear tree

On the twelfth day of Christmas
My true love gave to me
12 drummers drumming
Eleven pipers piping
Ten lords a leaping
Nine ladies dancing
Eight maids a milking
Seven swans a swimming
Six geese a laying
Five gold rings, badam-pam-pam
Four calling birds
Three French hens
Two turtle doves
And a partridge in a pear tree
*/
