use std::fs::File;
use std::io::ErrorKind;


fn main() {

    let _greeting_file_result = File::open("hello.txt").unwrap();

}
