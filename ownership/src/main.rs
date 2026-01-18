fn main() {
    let s1 = String::from("hello");
    println!("s1 before calculate_length: {}", s1);

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
