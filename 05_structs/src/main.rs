#![allow(unused)]

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32); // tuple structs
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone123"),
        active: true,
        sign_in_count: 1,
    };

    u1.active = false; // set user inactive (entire instance needs to be mut, no mut fields)

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername"),
        ..u1 // use rest from u1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // ! continue p.88

    let w1 = 30;
    let h1 = 50;
    println!(
        "The area of the rectangle is {} square pixels",
        area(30, 50)
    );

    let r1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of rectangle {:?} is {}", r1, area_r(&r1));
    println!("The area of rectangle {:?} is {}", r1, r1.area());

    let r1 = Rectangle {
        width: 30,
        height: 50,
    };
    let r2 = Rectangle {
        width: 10,
        height: 40,
    };
    let r3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can r1 hold r2? {}", r1.can_hold(&r2));
    println!("Can r1 hold r3? {}", r1.can_hold(&r3));

    let s = Rectangle::square(30);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_t(r: (u32, u32)) -> u32 {
    r.0 * r.1
}

fn area_r(r: &Rectangle) -> u32 {
    r.width * r.height
}
