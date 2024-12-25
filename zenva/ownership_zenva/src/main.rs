fn main() {
    let s1 = String::from("hello");

    let s2 = s1.clone();

    println!("{s2} world");
    println!("{s1} world");

    let name = String::from("rob");
    let name = print_name(name);
    println!("{name}");

//reference

let name = String::from("rob");
print_greeting(&name);
println!("{name}");
    //part 2

    //slice
let message = String::from("Hello world");

let hello = &message[0..5];
println!("{hello}");



let array = [1,2,3,4,5];

let slice = &array[1..4];
for num in slice {
    println!("{num}");
}
}

fn print_name(name: String) -> String {
    println!("welcome {}", name);
    name
}

fn print_greeting (name : &String) {
    println!("Welcome {}", name);
}