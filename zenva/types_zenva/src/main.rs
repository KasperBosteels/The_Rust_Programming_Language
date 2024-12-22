fn main() {

let mut value = 9;

value = 10;
println!("this value is {}", value);

let small_value :i8 = 100;

let sample_float:f32  = -700.23;
println!("the  int value is {}", small_value);

println!("the value is {}", sample_float);




let numbers:[i32;10] = [1,2,3,4,5,6,7,8,9,42];
println!("element at 0 {}", numbers[0]);

let person = ("alice", 30, 5.4);
println!("name is {}", person.0);

print!("age {}", person.1);

println!("height {}", person.2);


const pi:f32 = 3.14;
println!("const pi is {}", pi);

}
