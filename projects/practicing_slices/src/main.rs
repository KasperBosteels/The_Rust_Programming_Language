fn main() {
    let s = String::from("hello world hello1");
    let word = first_word(&s);
    println!("The first word is: {}", word);
    let second_word = second_word(&s);
    println!("The second word is: {}", second_word);
    slicing();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // convert string to array of bytes
    for (i, &item) in bytes.iter().enumerate() { // iter returns each element in a collection and enumerate wraps the result of iter and returns each element as part of a tuple instead
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..];
        }
    }
    &s[..]
}
fn slicing () {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("  {} {}", hello, world);
}