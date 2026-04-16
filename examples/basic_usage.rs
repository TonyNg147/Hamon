use hamon::prelude::*;

fn main() {
    let result = Builder::new(10)
        .step(Add { val: 5 })
        .step(|x| x * 2) // Using the blanket closure implementation
        .step(ToStringDec)
        .build();

    println!("Final Result: {}", result);
}
