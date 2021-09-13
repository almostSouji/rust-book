#![allow(unused)]

enum IpAdrrKind {
    V4,
    V6,
}

struct IpAddr1 {
    kind: IpAdrrKind,
    address: String,
}

enum IpAddr2 {
    V4(String),
    V6(String),
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let f = IpAdrrKind::V4;
    let s = IpAdrrKind::V6;
    route(f);
    route(s);

    let home = IpAddr1 {
        kind: IpAdrrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr1 {
        kind: IpAdrrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr2::V4(String::from("128.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));

    let home = IpAddr::V4(128, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));

    let sn = Some(5);
    let ss = Some("a string");
    let no_number: Option<i32> = None;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8 = 0u8;
    match some_u8 {
        1 => println!("one"),
        2 => println!("three"),
        3 => println!("five"),
        _ => (), // matches any
    }

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

fn route(ip_kind: IpAdrrKind) {}
