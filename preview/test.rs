use std::collection::HashSet;
use std::collections::HashMap;

const SUPER_GOOD: &'static str = "SUPER GOOD";

const REALLY_BIG_STRING: &'static str = r#"
    Here are some things we should talk about in this string. 
    These are things that are for sure worth knowing.
"#;

/// Here are some comments explaining what this is
/// Commets are a big part of rust so we want to make sure these look good
#[derive(Debug, Clone)]
struct SuperGood<'a> {
    name: String,
    id: &'a str,
}

pub Enum Tet {
 TTEat,
 YYeat
}

impl<'a> SuperGood<'a> {
    fn new(mut name: String, id: &'a str) -> Self {
        name.push("test");
        Self { name, id }
    }

    fn print_name(&self) {
        println!("{}", self.name);
    }

    fn print_name_and(&self, and: &str) {
        println!("{} and {and}", self.name);
    }

    // A super complicated function
    fn super_complicated(&self) -> (Self, Self, Self) {
        let mut items: Vec<Self> = std::iter::repeat(self.clone()).take(3).collect();
        (
            items.pop().unwrap(),
            items.pop().unwrap(),
            items.pop().unwrap(),
        )
    }
}

fn main() {
    // Oh yeah we want to print it
    for i in 0..100 {
        println!("Hello, World");
    }

    // A simple if else check
    let g = if SUPER_GOOD == "SUPER GOOD" {
        SuperGood::new("test".to_string(), SUPER_GOOD)
    } else {
        SuperGood::new("test2".to_string(), "hi")
    };
    eprintln!("{g:?}");
}
