use std::sync::{Arc, Mutex};
// Mutex: mutual exclusion - lock principle
// Arc: Atomic RC - concurrency safe reference count
use std::thread;

pub fn main() {
	simple_mutex();

	let counter = Arc::new(Mutex::new(0));
	let mut handles = vec![];

	for _ in 0..10 {
		let counter = Arc::clone(&counter);
		let handle = thread::spawn(move || {
			let mut num = counter.lock().unwrap();
			*num += 1;
		});
		handles.push(handle);
	}

	for handle in handles {
		handle.join().unwrap();
	}

	println!("Result: {}", *counter.lock().unwrap());
}

fn simple_mutex() {
	let m = Mutex::new(5);

	{
		let mut num = m.lock().unwrap();
		*num = 6;
	}

	println!("m = {:?}", m);
}
