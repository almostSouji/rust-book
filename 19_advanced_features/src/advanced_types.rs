#![allow(unused)]
type Kilometers = i32; // type alias

pub fn main() {
	let x: i32 = 5;
	let y: Kilometers = 5;

	println!("x + y = {}", x + y);

	// type alias does not have type-checking benefits of newtypes
	// mostly used to avoid repetition
	let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

	type Thunk = Box<dyn Fn() + Send + 'static>;
	let f: Thunk = Box::new(|| println!("hi"));
}

// "Function that returns never"
// fn bar() -> ! {
//
// }
