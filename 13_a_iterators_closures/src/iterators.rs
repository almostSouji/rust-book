fn iterators() {
	let v1 = vec![1, 2, 3];
	// create iterator, does not consume anything yet
	let v1_iter = v1.iter();
	// iter_mut: mutable elements
	// into_iter: consume and take ownership

	for val in v1_iter {
		// consumes elements
		println!("Got: {}", val);
	}

	let v1: Vec<i32> = vec![1, 2, 3];
	v1.iter().map(|x| x + 1); // iterators are lazy, nothing happens, because no elements are consumed

	let v1: Vec<i32> = vec![1, 2, 3];
	let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); // now elements are consumed
	assert_eq!(v1, v2);
}

#[derive(PartialEq, Debug)]
struct Shoe {
	size: u32,
	style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
	shoes.into_iter().filter(|s| s.size == shoe_size).collect()
	// takes ownership
}

#[test]
fn filter_by_size() {
	let shoes = vec![
		Shoe {
			size: 10,
			style: String::from("sneaker"),
		},
		Shoe {
			size: 13,
			style: String::from("sandal"),
		},
		Shoe {
			size: 10,
			style: String::from("boot"),
		},
	];

	let in_my_size = shoes_in_my_size(shoes, 10);
	assert_eq!(
		in_my_size,
		vec![
			Shoe {
				size: 10,
				style: String::from("sneaker"),
			},
			Shoe {
				size: 10,
				style: String::from("boot"),
			},
		]
	)
}

fn capture_env() {}

#[test]
fn iterator_sum() {
	let v1 = vec![1, 2, 3];
	let v1_iter = v1.iter();
	let total: i32 = v1_iter.sum(); // takes ownership
	assert_eq!(total, 6);
}

struct Counter {
	count: u32,
}

impl Counter {
	fn new() -> Counter {
		Counter { count: 0 }
	}
}

impl Iterator for Counter {
	type Item = u32;

	fn next(&mut self) -> Option<Self::Item> {
		self.count += 1;
		if self.count < 6 {
			Some(self.count)
		} else {
			None
		}
	}
}

#[test]
fn calling_next_directly() {
	let mut counter = Counter::new();

	assert_eq!(counter.next(), Some(1));
	assert_eq!(counter.next(), Some(2));
	assert_eq!(counter.next(), Some(3));
	assert_eq!(counter.next(), Some(4));
	assert_eq!(counter.next(), Some(5));
	assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
	let sum: u32 = Counter::new()
		.zip(Counter::new().skip(1)) // returns none if either of the components is None, so (5, None) is not produced
		.map(|(a, b)| a * b)
		.filter(|x| x % 3 == 0)
		.sum();
	assert_eq!(18, sum);
}
