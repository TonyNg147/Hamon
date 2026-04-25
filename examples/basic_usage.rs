use hamon::errors::Result;
use hamon::prelude::*;
// A decorator that adds up an amout to the current value (as i32)
struct Add(i32);

// A decorator that multiplies up an amout to the current value (as i32)
struct Multiply(i32);

// A decorator converts the integer value to String value
struct ToString;

impl Decorator<i32, i32> for Add {
    fn produce(&mut self, input: i32) -> Result<i32> {
        println!("{:<10}: previous value was {}", "[ADD]", input);
        Ok(self.0 + input)
    }
}

impl Decorator<i32, i32> for Multiply {
    fn produce(&mut self, input: i32) -> Result<i32> {
        println!("{:<10}: previous value was {}", "[MULTIPLY]", input);
        Ok(self.0 * input)
    }
}

impl Decorator<i32, String> for ToString {
    fn produce(&mut self, input: i32) -> Result<String> {
        println!("{:<10}: previous value was {}", "[TOSTRING]", input);
        Ok(format!("{}", input))
    }
}

fn main() {
    let engine = Builder::new(10)
        .step(Add(2))
        .step(Multiply(10))
        .step(ToString);

    println!("{:<10}: {}", "[DEPTH]", engine.get_index());
    println!("{:<10}: {:?}", "[FINAL]", engine.collect());
}
