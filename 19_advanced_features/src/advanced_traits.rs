use std::ops::Add;

pub trait Iterator {
	type Item; // associated type

	// does not need to be annotated each time
	// would be the case with generics!
	fn next(&mut self) -> Option<Self::Item>;
}

#[derive(Debug, PartialEq)]
struct Point {
	x: i32,
	y: i32,
}

struct Millimeters(u32);
struct Meters(u32);

// implementing add for Millimeters if RHS is meters
impl Add<Meters> for Millimeters {
	type Output = Millimeters;

	fn add(self, other: Meters) -> Millimeters {
		Millimeters(self.0 + (other.0 * 1000))
	}
}

// RHS: right hand side
// if not specified, RHS will be sel
// Syntax: <PlaceholderType=ConcreteType>
trait Add2<RHS = Self> {
	type Output;

	fn add(self, rhs: RHS) -> Self::Output;
}

impl Add for Point {
	type Output = Point;

	fn add(self, other: Point) -> Point {
		Point {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}

pub fn main() {
	assert_eq!(
		Point { x: 1, y: 3 } + Point { x: 2, y: 3 },
		Point { x: 3, y: 6 }
	);

	flying();
	woof();
	prints();

	let w = Wrapper(vec![String::from("hello"), String::from("world")]);
	println!("w = {}", w);
}

trait Pilot {
	fn fly(&self);
}

trait Wizard {
	fn fly(&self);
}

struct Human;

impl Pilot for Human {
	fn fly(&self) {
		println!("This is your captain speaking")
	}
}

impl Wizard for Human {
	fn fly(&self) {
		println!("Up!")
	}
}

impl Human {
	fn fly(&self) {
		println!("*waving arms furiously*")
	}
}

fn flying() {
	let person = Human;
	person.fly(); // by default uses fn directly on the type
	Pilot::fly(&person); // explicit calls to trait versions
	Wizard::fly(&person);
}

trait Animal {
	fn baby_name() -> String;
}

struct Dog;

impl Dog {
	fn baby_name() -> String {
		String::from("Spot")
	}
}

impl Animal for Dog {
	fn baby_name() -> String {
		String::from("puppy")
	}
}

fn woof() {
	println!("A baby dog is called a {}", Dog::baby_name());
	// Syntax: <Type as Trait>::function(receiver_if_method, next_arg, ...)
	println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // disambiguate with fully qualified syntax
}

use std::fmt;

trait OutlinePrint: fmt::Display {
	fn outline_print(&self) {
		let output = self.to_string();
		let len = output.len();
		println!("{}", "*".repeat(len + 4));
		println!("*{}*", " ".repeat(len + 2));
		println!("* {} *", output);
		println!("*{}*", " ".repeat(len + 2));
		println!("{}", "*".repeat(len + 4));
	}
}

impl fmt::Display for Point {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

impl OutlinePrint for Point {}

fn prints() {}

//newtype

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "[{}]", self.0.join(", "))
	}
}
