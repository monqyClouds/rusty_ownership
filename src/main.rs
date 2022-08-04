fn main() {
    let mut s = String::from("hello world!");

    let _word = first_word(&s);

    let _word = first_word(&s[..]);
    let _word = first_word(&s[0..6]);

    let string_literal = "hello world";

    let _word = first_word(&string_literal[..]);
    let word = first_word(&string_literal[0..6]);

    println!("first word is '{}'", word);

    s.clear();


}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// fn change(some_string: &mut String) {
//     some_string.push_str(", world!");
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }
