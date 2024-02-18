fn main() {
    let mut condition = true;
    loop {
        let y = if condition {23} else {1};
        let x = plus_one(y);
    
        if x > 5 {
            println!("Condition met...");
        } else if x % 2 == 0 {
            println!("Condition 2 met...");
            break;
        }else {
            println!("Conditions not met...");
        }
        condition = !condition;
    }
}

fn plus_one (x:i8) -> i8 {
    x+1
}