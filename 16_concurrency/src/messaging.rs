use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn main() {
	// tx (transmitter) = sending, rx (receiver) = receiving
	let (tx, rx) = mpsc::channel();
	let tx1 = mpsc::Sender::clone(&tx);

	thread::spawn(move || {
		// move, so thread owns tx and can send
		let vals = vec![
			String::from("hi"),
			String::from("from"),
			String::from("the"),
			String::from("thread"),
		];

		for v in vals {
			tx1.send(v).unwrap();
			thread::sleep(Duration::from_secs(1));
		}
	});

	thread::spawn(move || {
		// move, so thread owns tx and can send
		let vals = vec![
			String::from("more"),
			String::from("messages"),
			String::from("for"),
			String::from("you"),
		];

		for v in vals {
			tx.send(v).unwrap();
			thread::sleep(Duration::from_secs(1));
		}
	});

	for received in rx {
		println!("Got: {}", received);
	}
}
