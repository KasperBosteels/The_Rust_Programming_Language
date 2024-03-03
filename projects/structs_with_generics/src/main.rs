struct Point<T> {
    x: T,
    y: T,
}

struct PointWithMoreTypes<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 11};
    let float = Point { x: 21.11, y: 4.0};
    // let wont_work = Point { x:21, y: 4.0};
    let integerAndFloat = PointWithMoreTypes { x:21, y:32.11};
}