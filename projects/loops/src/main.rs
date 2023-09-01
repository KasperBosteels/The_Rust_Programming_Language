fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("loop = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count: {}", count);
    while_loop();
    while_loop_trough_array();
    for_loop_trough_array();
}

fn while_loop (){
    let mut number = 3;
    
    while number != 0 {
        println!("{number}");
        number -=1;
    }
    println!("LIFTOFF!!");
}

fn while_loop_trough_array (){
    let a = [10, 30, 40, 50,60,11];
    let mut index = 0;
    
    while index < 5 {
        println!("the value is {}",a[index]);
        index +=1;
    }
}
//for loops are preffered to loop trough arrays to limit index array posibility
fn for_loop_trough_array (){
    let a = [1,2,3,4,5,6,7,8,9,11];
    
    for element in a{
        println!("the value is {element}");
    }
    
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!");
}