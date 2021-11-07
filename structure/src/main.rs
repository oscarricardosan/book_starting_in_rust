struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
struct User2 {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
struct Empty {} //Unit-Like Structs Without Any Fields


fn main() {
    let mut user1 = User {
        active: true,
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        sign_in_count: 1,
    };
    println!("Hello, {}!", user1.email);

    user1.email = String::from("anotheremail@example.com");
    println!("Hello again, {}!", user1.email);

    let user_1 = build_user(String::from("oscar.sanche@savne.net"), String::from("Oscar"));
    println!("Hello again, {}!", user_1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1  //The rest fields are equal to user1
    };
    println!("Hello again, {}!", user2.email);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(255, 0, 255);
    let origin = Point(0, -50, 50);
    println!("Hello again, {}!", black.0);

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
