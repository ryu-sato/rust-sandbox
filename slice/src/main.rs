use std::io;

fn main() {
    println!("Please input some sentence.");

    let mut sentence = String::new();

    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read line");

    let first_word_len = first_word(&sentence);

    println!("You input: {}, first word: {}", sentence, &sentence[..first_word_len]);
}

fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("First word ends at index: {}", i);
            return i;
        }
    }

    println!("The whole string is a single word.");
    s.len()
}
