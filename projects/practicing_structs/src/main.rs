fn main() {
let mut user1 = User {
    active:true,
    username: String::from("Kasper Bosteels"),
    email: String::from("123@mail.com"),
    sign_in_count: 1,
};
    user1.email = String::from("312@hotmail.com");
    let mut user2 = build_user(user1.username, user1.email);
    user2.active = user1.active;
    user2.sign_in_count = user1.sign_in_count;
    let user3 = User {
        active: false,
        ..user2
    };
let white = Color(255, 255, 255);
}

fn build_user (username:String, email:String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
struct Color(i32, i32, i32);
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}