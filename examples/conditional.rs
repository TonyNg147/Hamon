use hamon::prelude::*;

pub struct Add {
    pub val: i32,
}

impl Decorator<i32, i32> for Add {
    fn produce(&mut self, previous: i32) -> i32 {
        self.val + previous
    }
}

fn main() {
    let result1 = Builder::new(10)
        .step(Add { val: 5 }.when(|v| *v > 10)) // Predicate is evaluated to False then the decorator will be discarded.
        .build(); // Final Result: 10

    let result2 = Builder::new(10)
        .step(Add { val: 5 }.when(|v| *v > 4)) // Predicate is evaluated to True then the decorator will be kept.
        .build(); // Final result: 15

    println!("Final value1 will be {result1:?}");
    println!("Final value1 will be {result2:?}");
}
