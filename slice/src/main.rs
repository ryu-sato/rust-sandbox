use std::io;

fn main() {
    println!("Please input some sentence.");

    let mut sentence = String::new();

    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read line");

    let first_word = first_word(&sentence);

    println!("You input: {}, first word: {}", sentence, first_word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("First word ends at index: {}", i);
            return &s[0..i];
        }
    }

    println!("The whole string is a single word.");
    &s[..]
}
