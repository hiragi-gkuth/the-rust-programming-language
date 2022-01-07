fn main() {
    
}

fn other_type_slice() {
    let a = [0, 1, 2, 3, 4];
    let slice = &a[1..4];

    assert_eq!(slice, &[1, 2, 3]);
}


fn string_type_and_string_literal() {
    let my_string = String::from("hello world");

    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn last_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().rev().enumerate() {
        if item == b' ' {
            return &s[s.len()-i..s.len()]
        }
    }

    &s[..]
}