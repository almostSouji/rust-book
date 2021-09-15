use std::fmt::Display;

pub fn lt() {
	let s1 = String::from("abcd");
	let s2 = "xyz";

	let result = longest(s1.as_str(), s2);
	println!("The longest string is {}", result);

	let novel = String::from("Call me Ishamel. Some years ago...");
	let first_sentence = novel.split('.').next().expect("Could not find a .");
	let i = ImportantExcerpt {
		part: first_sentence,
	};
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}

struct ImportantExcerpt<'a> {
	part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
	fn annoucne_and_return_part(&self, announcement: &str) -> &str {
		println!("Attention please: {}", announcement);
		self.part
	}
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
	T: Display,
{
	println!("Announcement! {}", ann);
	if x.len() > y.len() {
		x
	} else {
		y
	}
}
