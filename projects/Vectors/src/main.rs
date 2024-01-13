fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(7);
    v.push(100);

    let ve  = vec![1,2,3,4,5];
    let third :&i32 = &ve[2];
    println!("The third element is {third}");
    let third: Option<&i32> = ve.get(100);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![100,32,57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![11,32,55];
    for i in &mut v {
        println!("{i}");
        *i *= 10;
        println!("{i}");
    }

    enum spread_sheet_cell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        spread_sheet_cell::Float(3),
        spread_sheet_cell::Int(11),
        spread_sheet_cell::Text("Bizerba");
    ];
}
