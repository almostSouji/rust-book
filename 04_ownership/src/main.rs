#![allow(unused)]
fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);

    let x = 5;
    let y = x; // copied
    let s1 = String::from("hello");
    let s2 = s1;
    // moved s1 into s2 (!)
    // s1 no longer valid after this point

    let s1 = String::from("hello");
    let s2 = s1.clone(); // deep copy, s1 is stil valid

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); // valid, integer is copy

    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s is no longer valid, the function took ownership

    let x = 5; // x comes into scope
    makes_copy(x);
    // x is still valid, i32 is copy
    // x goes out of scope, then s. But s's value was moved

    let s1 = gives_ownership(); // moves value into s1
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into to fct, moved into s3

    // s3 goes out of scope and is dropped
    // s2 goes out of scope but nothing happens (already moved)
    // s1 goes out of scope and is dropped

    let (s4, s4l) = calculate_length(String::from("hello"));
    println!("s3 {} is {} long", s4, s4l);
    let s1 = String::from("hello");
    println!("s1 {} is {} long", s1, calc_len(&s1)); // &s1 is reference
                                                     // s1 is still valid

    let s = String::from("hello world");
    let hello = &s[0..5]; // string slices
    let world = &s[6..11];
    let slice = &s[0..2]; // identical to next
    let slice = &s[..2];
    let slice = &s[0..s.len()]; // identical to next
    let slice = &s[0..]; // identical to
    let slice = &s[..];

    let mut s = String::from("hello world");
    let word = first_word(&s);

    let a = [1, 2, 3, 4, 5];
    let s = &a[1..3]; // &[i32] slice
}

fn takes_ownership(s: String) {
    println!(
        "S (ownership taken, drop is called, memory is freed): {}",
        s
    );
}

fn makes_copy(i: i32) {
    println!("I (was a copy, nothing special happens): {}", i);
}

fn gives_ownership() -> String {
    let s = String::from("hello"); // s comes into scope
    s // s is returned and moves out of the calling function
}

fn takes_and_gives_back(s: String) -> String {
    s
}

// multiple return values
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calc_len(s: &String) -> usize {
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
