use hamon::errors::Result;
use hamon::prelude::*;
// A decorator that adds up an amout to the current value (as i32)
struct Add(i32);

impl Decorator<i32, i32> for Add {
    fn produce(&mut self, input: i32) -> Result<i32> {
        println!("{:<10}: previous value was {}", "[ADD]", input);
        Ok(self.0 + input)
    }
}

fn main() {
    let engine = Builder::new(10).step(Add(2)).step(|x| {
        println!("{:<10}: previous value was {}", "[CLOSURE]", x);
        Ok(10 * x)
    });

    println!("{:<10}: {}", "[DEPTH]", engine.get_index());
    println!("{:<10}: {:?}", "[FINAL]", engine.collect());
}
