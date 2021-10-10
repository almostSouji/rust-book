#![allow(unused)]

fn main() {
    // if let
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as background color");
        } else {
            println!("Using orange as background color");
        }
    } else {
        println!("Using blue as background color");
    }

    // while let
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top)
    }

    // for loops
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let
    let (x, y, z) = (1, 2, 3);

    //fn parameters
    let point = (3, 5);
    print_coordinates(&point);

    //match literals
    let x = 1;
    match x {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("anything"),
    }

    //match multi pattern
    let x = 1;
    match x {
        1 | 2 => println!("1 or 2"),
        3 => println!("3"),
        _ => println!("anything"),
    }

    //match ranges
    let x = 1;
    match x {
        1..=5 => println!("1 through 5"), // inclusive range
        _ => println!("anything"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    //destructuring structs
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On nether axis: ({}, {})", x, y),
    }

    //match guards

    let n = Some(4);
    match n {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // @ bindings

    let m = Message::Hello { id: 5 };
    match m {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

enum Message {
    Hello { id: i32 },
}
