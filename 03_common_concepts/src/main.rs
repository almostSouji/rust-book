#![allow(unused)]
const MAX_POINTS: u32 = 100_00;

fn main() {
    let mut x = 5;
    x = 6;
    let x = x + 1; // shadowed previous mut with non-mut
    println!("The value of x is: {}", x);

    let spaces = "    ";
    let spaces = spaces.len();

    let x = 2.0; // f64 (default float)
    let y: f32 = 3.0; // f32

    let t = true;
    let f: bool = false;

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    let c = 'z';
    let crab = '🦀';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let five_hundred = tup.0;
    let six_point_four = tup.1;

    let arr = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    let first = a[0];

    some_function(5);

    let y = {
        let x = 3;
        x
    };

    // same type in both branches is required
    let n = if y < 3 { 5 } else { 6 };
}

fn some_function(x: i32) {
    println!("Some function has been called with {}", x);
}

fn five() -> i32 {
    5
}

fn divisible(n: i32) {
    if n % 4 == 0 {
        println!("divisible by 4");
    } else if n % 3 == 0 {
        print!("divisible by 3");
    } else if n % 2 == 0 {
        print!("divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2")
    }
}

fn countdown() {
    let mut n = 3;
    while n != 0 {
        println!("{}!", n);
        n -= 1;
    }
    print!("Liftoff!");
}

fn loop_collections() {
    let a = [1, 2, 3, 4, 5];
    for element in a.iter() {
        println!("{}!", element);
    }
    print!("Liftoff!");
}
