#![allow(unused)]

use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

mod iterators;

struct CacherPlus<T, K, V>
where
    T: Fn(&K) -> V,
    K: Eq + Hash,
{
    calculation: T,
    values: HashMap<K, V>,
}

impl<T, K, V> CacherPlus<T, K, V>
where
    T: Fn(&K) -> V,
    K: Eq + Hash,
{
    fn new(calculation: T) -> CacherPlus<T, K, V> {
        CacherPlus {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> &V {
        use std::collections::hash_map::Entry;
        match self.values.entry(arg) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let v = (self.calculation)(entry.key());
                entry.insert(v)
            }
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                v
            }
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn capture() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y)); // x is captured from the environment
}

fn capture2() {
    let x = vec![1, 2, 3];
    // "move" keyword force-consume the variable, passing ownership to the closure
    let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x); // cannot use x here, as it'd require ownership

    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

// Closure traits:
// FnOnce -> closure consumes variable, takes ownership
// FnMut -> mutable borrow
// Fn -> borrows immutably
