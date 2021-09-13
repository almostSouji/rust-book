#![allow(unused)]

mod front_of_house;

pub use crate::front_of_house::hosting;

mod back_of_house {
    // all variants are public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // required, else we couldn't create a Breakfast! (Breakfast has private field)
        // otherwise, wouldn't be constructable
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist(); // works because of the use statement
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries") // not allowed, fruit is not pub
}

// full path for structs, one short for methods (so you have to specify the parent)
use std::collections::HashMap;

use std::fmt::Result;
use std::io::Result as IoResult; // renaming with use as

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

use std::collections::*;
use std::{cmp::Ordering, io}; // nested use // glob operator, use all
