#![allow(unused)]
use std::collections::HashMap;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v = vec![1, 2, 3];
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; // might panic if out of range

    match v.get(2) {
        Some(third) => println!("Third element is {}", third),
        None => println!("There is no third element"),
    }

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // add 50 to each element (deref)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 is moved, s2 isn't

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); //overwrites

    scores.entry(String::from("Blue")).or_insert(50); // only insert if not present, else return old

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    let v = vec![2, 2, 5, 5, 5, 5, 5, 6, 6, 6];
    println!("Mean of {:?}: {}", v, mean(&v));
    println!("Median of {:?}: {}", v, median(&v));
    println!("Mode of {:?}: {}", v, mode(&v));
    println!("Pig latin (first): {}", pig_latin("first"));
    println!("Pig latin (apple): {}", pig_latin("apple"));

    start_interface();
} // dropping a vector drops its elements

fn mean(vals: &Vec<i32>) -> f32 {
    let s: i32 = vals.iter().sum();
    s as f32 / vals.len() as f32
}

fn median(vals: &Vec<i32>) -> f32 {
    let mut s = vals.to_vec();
    s.sort();
    let i = vals.len() / 2;
    if vals.len() % 2 == 0 {
        mean(&vec![s[i - 1], s[i + 1]])
    } else {
        vals[i] as f32
    }
}

fn mode(vals: &Vec<i32>) -> i32 {
    let mut m = HashMap::new();
    for v in vals {
        let score = m.entry(v).or_insert(0);
        *score += 1;
    }
    let mut most = -1;
    for v in vals {
        if m[v] > most {
            most = m[v];
        }
    }
    most
}

const VOWEL: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn pig_latin(s: &str) -> String {
    let mut res = String::new();

    let chars: Vec<char> = s.chars().collect();
    let first = match chars.get(0) {
        Some(val) => val,
        None => {
            return String::from(s);
        }
    };

    if VOWEL.contains(first) {
        return String::from(s) + "-hay";
    }

    let mut first_cons_index = None;
    for (i, c) in chars.iter().enumerate() {
        if !VOWEL.contains(&c) && first_cons_index == None {
            first_cons_index = Some(i);
        } else {
            res.push(*c);
        }
    }

    match first_cons_index {
        Some(i) => {
            res.push('-');
            res.push(chars[i]);
            res += "ay";
        }
        None => {
            return String::from(s);
        }
    }
    res
}

use std::io;
fn start_interface() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read line");
        let inp: Vec<&str> = inp.split_whitespace().collect();
        match inp.get(0) {
            None => continue,
            Some(&"add") => {
                let empl = match inp.get(1) {
                    Some(s) => s,
                    None => {
                        println!("Missing employee!");
                        continue;
                    }
                };
                let dept = match inp.get(3) {
                    Some(s) => s,
                    None => {
                        println!("Missing department!");
                        continue;
                    }
                };

                let current = employees.entry(String::from(*dept)).or_insert(vec![]);
                current.push(String::from(*empl));
            }
            Some(&"show") => {
                for (k, v) in &employees {
                    println!("Employees in department {}:", k);
                    let mut copy = v.to_vec();
                    copy.sort();
                    for e in &copy {
                        println!("- {}", e);
                    }
                }
            }
            Some(_) => continue,
        }
    }
}
