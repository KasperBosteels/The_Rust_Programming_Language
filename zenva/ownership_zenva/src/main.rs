fn main() {
    let s1 = String::from("hello");

    let s2 = s1.clone();

    println!("{s2} world");
    println!("{s1} world");

    let name = String::from("rob");
    let name = print_name(name);
    println!("{name}");
}

fn print_name(name: String) -> String {
    println!("welcome {}", name);
    name
}
