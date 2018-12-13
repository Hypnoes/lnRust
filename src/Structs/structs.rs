struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

fn mv_user(email: String, username: String, user1: User) -> User {
    User {
        email,
        username,
        ..user1
    }
}

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area($self) -> u32 {
        self.width * self.height
    }

    fn can_hold($self, rect2: &Rectangle) -> bool {
        if &self.area() >= rect2.area() {
            true
        } else {
            false
        }
    }
}

fn main() {
    let rect1 = Rectangle { width: 10, height: 20 };
    let rect2 = Rectangle { width: 20, height: 40 };

    if rect2.can_hold(&rect1) {
        println!("rect2 can hold rect1");
    } else {
        println!("rect2 can't hold rect1");
    }
}