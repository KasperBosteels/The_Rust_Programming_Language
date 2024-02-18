fn main() {
    let mut s = String::new();

    let data = "Initial Contents";
    let s = data.to_string();

    //the methode also works on a literal directly:
    let s = "Initial Contents".to_string();
    let s  = String::from("Initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");


    //using + format
    let s1 = String::from("Hello");
    let s2 = String::from("world");
    let s3 = s1 + " " + &s2;
    println!("{s3}");

}
