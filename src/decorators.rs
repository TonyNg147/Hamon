use crate::Decorator;

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
pub struct ToStringDec;

impl Decorator<i32, String> for ToStringDec {
    fn produce(&mut self, previous: i32) -> String {
        format!("Value: {previous}")
    }
}
