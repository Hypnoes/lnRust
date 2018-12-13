fn main() {
    let s1 = String::from("hello world");

    let s2 = s1[..5];

    let s3 = &s2[..];

    println!("{}\n{}", s2, s3);
}

// move
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    return (s, length);
}

// borrow
fn change(some_string: &String) -> String {
    let rts = some_string.to_string() + ", world!";
    return rts;
}

/*
 * The Slice Type
 */
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}