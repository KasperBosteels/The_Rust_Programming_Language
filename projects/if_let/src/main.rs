enum State{
    Antwerp,
    NewYork,
    Boedapesht
}

enum Coin{
    Cent(i16),
    Quarter(State),
    Euro(u8)
}
fn main() {
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("The maximum is to be {max}"),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximum from if let is to be {max}")
    }
    let coin = Coin::Quarter(State::NewYork);
    let mut count = 0;
    match coin {
        Coin::Quarter(state)=> println!("State quarter from {state}"),
        _ => count +=1,
    }
    if let Coin::Quarter(state) = coin {
        println!("This quarter is from the state {state}");
    }else{
        count+=1;
    }
}
