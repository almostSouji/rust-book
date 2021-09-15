#![allow(unused)]
#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!(
                "Guess value must be greater than or euqal to 100, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}", name)
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)] // not compuiled with cargo build/run
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let res = greeting("Carol");
        assert!(
            res.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            res
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")] // optional substring of the panic message
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn this_test_will_pass() {
        let v = prints_and_returns_10(4);
        assert_eq!(v, 10);
    }

    #[test]
    #[ignore] // only run if explicitly requested with --ignored
    fn this_test_will_fail() {
        // only failing tests print stdout, --nocatpure enables for passing tests
        let v = prints_and_returns_10(8);
        assert_eq!(v, 5);
    }

    // subset of tests by name:
    // single test: cargo test <name>
    // multile tests: argo test <namepart>
}
