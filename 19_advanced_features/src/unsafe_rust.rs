#![allow(unused)]

// UNSAFE SUPERPOWERS:
// Deref a raw pointer
// Call an unsfae function or method
// Access or modify a mutable static variable
// Implement an unsafe trait

static HELLO_WORLD: &str = "Hello, World!";
static mut COUNTER: u32 = 0;

pub fn main() {
	deref_raw_pointer();
	println!("name is: {}", HELLO_WORLD);
}

fn deref_raw_pointer() {
	let mut num = 5;
	let r1 = &num as *const i32;
	let r2 = &mut num as *mut i32;

	unsafe {
		println!("r1 is: {}", *r1);
		println!("r1 is: {}", *r2);

		danger(); // unsafe fn needs to be called within unsafe block
	}
}

unsafe fn danger() {}

use std::slice;

// safe abstraction to unsafe code
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
	let len = slice.len();
	let ptr = slice.as_mut_ptr();
	assert!(mid <= len); // asserting that pointers are always valid as necessary

	unsafe {
		(
			slice::from_raw_parts_mut(ptr, mid),
			slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
		)
	}
}

extern "C" {
	fn abs(input: i32) -> i32;
}

fn external_code() {
	unsafe { println!("Abs of -3 according to C: {}", abs(-3)) }
}

fn add_to_count(inc: u32) {
	unsafe {
		COUNTER += inc;
	}
}

fn counter() {
	add_to_count(3);

	unsafe {
		println!("COUNTER: {}", COUNTER);
	}
}

// unsafe traits

unsafe trait Foo {
	// methods go here
}

unsafe impl Foo for i32 {
	// method implementations go here
}
