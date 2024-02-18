fn main() {
    let s = String::from("hello");

    take_ownership(s);
    
    //println!("{}", s);
    //will throw error because s is no longer valid

    let x = 5;
    makes_copy(x);

    println!("{}", x); //x is still valued because i32 is primitive and copied
    return_values_and_scope();
}

fn take_ownership(some_string: String){ //String takes ownership over parameter
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){//i32 is a primitive type and it is copied
    println!("{}", some_integer);
}

fn return_values_and_scope (){
    let s1 = gives_ownership(); //gives_ownership moves its return value into s1

    let s2 = String::from("hello"); //s2 comes into scope

    let s3 = takes_and_gives_back(s2); //s2 is moved into takes_and_gives_back, which also moves its return value into s3
    println!("{}", s3);
    
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string //some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(some_string:String) -> String {

    some_string
}