use hamon::errors::Result;
use hamon::ext::DecoratorExt;
use hamon::prelude::*;

// Decorator that adds up an amount to existing value
pub struct Add {
    pub val: i32,
}

impl Decorator<i32, i32> for Add {
    fn produce(&mut self, previous: i32) -> Result<i32> {
        Ok(self.val + previous)
    }
}

// This example demonstrates the ability to conditionally perform
// a pipeline based on provided condition (predicate)
//
// As long as the condition is evaluated to true, the pipeline is allowed to continue its work.
fn main() {
    let pl1 = Builder::new(10)
        .step(Add { val: 5 }.when(|_v| true)) // Predicate is evaluated to False then the decorator will be discarded.
        .collect(); // Ok(15)

    let pl2 = Builder::new(10)
        .step(Add { val: 5 }.when(|_v| false)) // Predicate is evaluated to True then the decorator will be kept.
        .collect(); // Ok(10)

    println!("{:<10}: produces output {:?}", "[PIPELINE1]", pl1);
    println!("{:<10}: produces output {:?}", "[PIPELINE2]", pl2);
}
