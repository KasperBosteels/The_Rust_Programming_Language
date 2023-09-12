fn main() {
    let s = String::from("hello world hello1");
    let word = first_word(&s);
    println!("The first word is: {}", word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert string to array of bytes
    for (i, &item) in bytes.iter().enumerate() { // iter returns each element in a collection and enumerate wraps the result of iter and returns each element as part of a tuple instead
        if item == b' ' {
            return i;
        }
    }
    s.len()
}