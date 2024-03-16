
enum Option<T> {
    Some(T),
    None,
}

enum Result<T,E>{
    Ok(T),
    Err(E),
}

struct Point<T> {
    x: T,
    y: T,
}

impl Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point {x:5, y:10};
    println!("p.x = {}", p.x());
}
