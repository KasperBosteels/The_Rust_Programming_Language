fn main() {
    let user1 = User {
        email: String::from("alice"),
        username: String::from("alice"),
        active: true,
        sign_in_count: 1,
        date_of_birth: String::from("01/01/2000"),
    };
    print_user_message(&user1);

    let red = Color(255, 0, 0);
    let violet = Color(238, 130, 238);
    let point = Point(1.0, 2.0, 3.0);

    let rec1 = Rectanle {
        width: 30,
        height: 50,
    };
    let rec1_rect = rec1.area();

    println!("Rectangle area: {}", rec1_rect);

    rec1.print_dimensions();
}

fn print_user_message(user: &User) {
    println!("User {} is active: {}", user.username, user.active);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
    date_of_birth: String,
}

struct Rectanle {
    width: u32,
    height: u32,
}

impl Rectanle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn print_dimensions(&self) {
        println!("Rectangle width: {}, height: {}", self.width, self.height);
    }
}

struct Color(i32, i32, i32);
struct Point(f64, f64, f64);
