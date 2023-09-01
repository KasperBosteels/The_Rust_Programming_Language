fn main() {
    let mut x = 5;
    println!("The value of X is: {}", x);
    x = 6;
    println!("The value of X is: {}", x);
    let x = 11;
    println!("{}",x);
    //shadowing variables
    {
        let x = x+11;
        println!("The shadowed value of X is: {}", x);
    }
    println!("The value of X is now unshadowed: {}", x);
}
