fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for chr in list {
        if largest > chr {
            largest = chr;
        }
    }

    largest
} 

fn Largest<T: std::cmp::PartialOrd>(list: &[T])  -> &T {
    let mut largest = &list[0];

    for item in list {
        if largest > item {
            largest = item;
        }
    }

    largest
}



fn main() {
    let number_list = vec![34,50,25,100,65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");

    let number_list = vec![21,33,54,100,65,34,89];
    let largest = largest_i32(&number_list);
    println!("The largest number is {largest}");

    let char_list = vec!['a','t','q','k','o'];
    let largest = Largest(&char_list);
    println!("The largest char is {largest}");
}



