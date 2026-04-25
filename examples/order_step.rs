use hamon::errors::Result;
use hamon::{builder::OrderedBuild, prelude::*, AllowStep};
use std::fmt::Debug;

struct Encyption;
#[derive(AllowStep)]
#[from(Encyption)]
struct Compression;

impl<T: Debug> Decorator<T, T> for Encyption {
    fn produce(&mut self, input: T) -> Result<T> {
        println!("{:<15}: previous value was {:?}", "[ENCRYPTION]", input);
        Ok(input)
    }
}

impl<T: Debug> Decorator<T, T> for Compression {
    fn produce(&mut self, input: T) -> Result<T> {
        println!("{:<15}: previous value was {:?}", "[COMPRESSION]", input);
        Ok(input)
    }
}

fn main() {
    // Given the network transferring, when a packet leaves one network to reach others
    // It needs to be Encypted before Compression.
    let _result = OrderedBuild::new(10)
        .step(Encyption)
        .step(Compression)
        .collect();
}
