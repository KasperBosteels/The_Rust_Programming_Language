#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other : &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main() {

    let width1 = 30;
    let height1 = 50;
    println!("The area of the rectangle is {} square pixels.", area(width1, height1));

    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area_tuple(rect1));

    let scale = 2;

    let rect2 = Rectangle { width: dbg!(30* scale), height: 50 };
    println!("The area of the rectangle is {} square pixels.", area_struct(&rect2));
    println!("The area of the rectangle is {} square pixels calculated with method", rect2.area());
    println!("rect2 is {:?}", rect2);
    dbg!(rect2);
    let rect3 = Rectangle { width: 10, height: 40 };
    let rect4 = Rectangle { width: 60, height: 45 };
    println!("rect4 can hold rect3? {}", rect4.can_hold(&rect3));
    println!("rect3 can hold rect4? {}", rect3.can_hold(&rect4));
}
fn area(width: u32, height: u32) -> u32 {
    width * height
}
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

