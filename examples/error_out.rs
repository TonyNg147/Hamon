use hamon::errors::{Context, Result};
use hamon::prelude::*;
// A decorator that adds up an amout to the current value (as i32)
struct Add(i32);

// A decorator that multiplies up an amout to the current value (as i32)
struct Multiply(i32);

// A decorator converts the integer value to String value
struct ToString(Option<String>);
// Convert back to INT
struct ToInt;

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
        if let Some(prefix) = &self.0 {
            Ok(format!("{}{}", prefix, input))
        } else {
            Ok(format!("{}", input))
        }
    }
}

impl Decorator<String, i32> for ToInt {
    fn produce(&mut self, input: String) -> Result<i32> {
        println!("{:<10}: previous value was {}", "[TOINT]", input);
        input
            .parse::<i32>()
            .step_err(format!("Cannot parse {input:?} to INT"))
    }
}

fn main() {
    let success_pl = Builder::new(10)
        .step(Add(2))
        .step(Multiply(10))
        .step(ToString(None))
        .step(ToInt);

    let failed_pl = Builder::new(10)
        .step(Add(2))
        .step(Multiply(10))
        .step(ToString(Some("test".into())))
        .step(ToInt);

    println!("{:<10}: {:?}", "[FINAL]", success_pl.collect());
    println!("{:<10}: {:?}", "[FINAL]", failed_pl.collect());
}
