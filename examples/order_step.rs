use std::fmt::Debug;

use hamon::{builder::OrderedBuild, prelude::*, AllowStep};

struct Encyption;
#[derive(AllowStep)]
#[from(Encyption)]
struct Compression;

impl<T: Debug> Decorator<T, T> for Encyption {
    fn produce(&mut self, input: T) -> T {
        println!("{:<15}===> Data is {input:?}", "Encryption");
        input
    }
}

impl<T: Debug> Decorator<T, T> for Compression {
    fn produce(&mut self, input: T) -> T {
        println!("{:<15}===> Data is {input:?}", "Compression");
        input
    }
}

fn main() {
    // Given the network transferring, when a packet leaves one network to reach others
    // It needs to be Encypted before Compression.
    let result = OrderedBuild::new(10)
        .step(Encyption)
        .step(Compression)
        .build();
}
