enum UsState {
    Alabama,
    Alaska,
    NewYork,
    Texas,
}
enum Coin {
    Penny,
    Nicker,
    Dime,
    Quarter(UsState),
}

//ways of using match(switch case)
fn value_in_cents(coin:Coin) -> u8 {
    match coin {
        Coin::Penny => {1}
        Coin::Nicker => {5}
        Coin::Dime => {10}
        Coin::Quarter(state) => {
            println!("{}",&format!("State quarter from {:#?}", state));
            25
        }
    }
}

//using match to check if value is None(null)
fn plus_one (x: Option<i32>) -> Option<i32>{
    match x
    {
        None => None,
        Some(i) => Some(i+1),
    }
}

//using other and functions is match
fn match_dice_roll (x:i8){
    match x
    {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        //catch all but no value to assign
        //_ => add_fancy_hat(),
        //how to catch all and execute nothing
        //_ => (),
    }
}
fn add_fancy_hat () {}
fn remove_fancy_hat () {}
fn move_player(num_spaces:i8) {}
fn main () {
    value_in_cents(Coin::Quarter(UsState::Alabama));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
