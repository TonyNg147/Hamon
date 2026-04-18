use hamon::prelude::*;

/// A decorator that adds a fixed value to an integer.
pub struct Add {
    pub val: i32,
}

impl Decorator<i32, i32> for Add {
    fn produce(&mut self, previous: i32) -> i32 {
        self.val + previous
    }
}

/// A decorator that converts an integer into a formatted String.
struct ToStringDec;

impl Decorator<i32, String> for ToStringDec {
    fn produce(&mut self, previous: i32) -> String {
        format!("Value: {previous}")
    }
}

fn main() {
    let result = Builder::new(10)
        .step(Add { val: 5 })
        .step(|x| x * 2) // Using the blanket closure implementation
        .step(ToStringDec)
        .build();

    println!("Final Result: {}", result);
}
