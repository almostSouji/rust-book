#![allow(unused)]
mod lifetimes;

struct Point<T> {
    x: T,
    y: T,
}

// only valid in f32 Point instances
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let nl = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&nl));

    let i = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    lifetimes::lt();
}

fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &n in list.iter() {
        if n > largest {
            largest = n;
        }
    }
    largest
}

pub trait Display {}
pub trait Clone {}
pub trait Debug {}

pub trait Summary {
    fn summarize(&self) -> String {
        // default implementation on the trait
        format!("(Read more from {}...)", self.summarize_author())
        // makes summarize_author required, because used in default (!)
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// implementing a trait
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        String::from(&self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn news() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        retweet: false,
        reply: false,
    };

    println!("{}", tweet.summarize());
}

fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
} // anything that implements Summary can be passed here

// long form of above syntactic sugar
fn notify1<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// require 2 traits to be implemented on the passed instance
fn notify2(item: impl Summary + Display) {
    println!("Breaking news! {}", item.summarize());
}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    3
}

// clearer version
fn some_function2<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    3
}

// impl return types
// only for single types
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
