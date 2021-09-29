use std::thread;
use std::time::Duration;

mod messaging;
mod shared_state;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap(); // make sure the spawned thread finishes before main exits
    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    move_closures();
    messaging::main();
    shared_state::main();
}

fn move_closures() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        // force closure to take ownership (of v) instead of inferring a borrow
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}
