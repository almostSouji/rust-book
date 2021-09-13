#![allow(unused)]
use std::fs;
use std::io;
use std::{fs::File, io::ErrorKind, io::Read};

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    shorts();
}

fn p() {
    panic!("crash and burn");
}

fn p1() {
    let v = vec![1, 2, 3];
    v[99];
}

fn shorts() {
    // will panic if the file doesn't exist
    let f = File::open("world.txt").unwrap();
    // will panic if the file doesn't exist, let's you provide custom error message
    let f = File::open("world.txt").expect("Failed to open world.txt");
}

fn read_username_from_file1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    // almost same as above, return error if not ok, else return ok val
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    // shorter with chaining
    let mut s = String::new();
    let mut f = File::open("hello.txt")?.read_to_string(&mut s);
    Ok(s)
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
