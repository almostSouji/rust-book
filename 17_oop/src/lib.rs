pub trait Draw {
	// traits instead of inheritance
	fn draw(&self);
}

pub struct Screen {
	pub components: Vec<Box<dyn Draw>>, // stand in, implements Draw trait
}

impl Screen {
	pub fn run(&self) {
		for component in self.components.iter() {
			component.draw();
		}
	}
}

pub struct Button {
	pub width: u32,
	pub height: u32,
	pub label: String,
}

impl Draw for Button {
	fn draw(&self) {
		// code to actually draw button
	}
}
