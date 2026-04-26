# Hamon (刃文)

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

**Hamon** is a zero-cost, type-level static decorator library for Rust. It allows you to compose complex data processing pipelines that are resolved entirely at compile time, eliminating the performance overhead of dynamic dispatch and heap allocation. Additionally, the sake of modularity this crate brings about ensuring for the ability to extension, testability and ease of writting code in Rustacean fashion.

## ⚔️ The Philosophy
Data transformation takes input and produces the output when it recurs several times requiring us to write the boilerplat code which seems to be tedious tasks, somewhat hard to test, maintain, and utimately doesn't reserve rooms for the extension. In practice, Logic onwer may bring up a pipeline step with massive logic underneath. Even the down-breaking of processing units emerges at the cure for simplicity, it deems to be adequate due to the disunion of what it would take and produce.

On top of that, In high-performance systems programming, every instruction counts. Traditional decorator patterns often rely on `Box<dyn Decorator>`, which incurs a "vtable tax"—the performance penalty of pointer chasing and inhibited compiler optimizations.

**Hamon** leverages **Monomorphization**. By using recursive generics, the entire execution chain is baked into the type system. This allows the LLVM optimizer to "see through" the abstraction, inlining logic and generating machine code that is as fast as a hand-written monolithic function.

## 🚀 Outstanding Features

- **Zero-Cost Abstractions**: No dynamic dispatch. Direct function calls are resolved at compile time.
- **Type-Level Validation**: The compiler verifies your pipeline. If a decorator doesn't fit the lineage, it won't compile.
- **Stack-First Efficiency**: Avoids heap fragmentation. Data remains in CPU registers and the stack for maximum throughput.
- **Occupancy for error-handling**: Transformations are not crossing through. They should be considered fallible. Pipeline will be ceased if encoutered an error.
- **Modular Maintainability**: Break 500-line "spaghetti" functions into small, isolated, and testable `Decorator` structs without losing performance.

## 📊 Performance Dojo

Benchmarks comparing a **50-step pipeline** across three implementation strategies:

| Implementation | Average Time | Memory Allocations | Speed Delta |
| :--- | :--- | :--- | :--- |
| **Monolithic Function** | 4.36 µs | 1 (Final Result) | — |
| **Hamon (Static)** | **4.98 µs** | **1 (Final Result)** | **Baseline** |
| **Traditional (Dynamic)** | 10.14 µs | 51 (Boxes + Vec) | **~2.0x Slower** |

*Hamon provides the modularity of a dynamic system with the raw speed of hand-optimized code.*

## 🛠️ Usage

Hamon uses a conservative, familiar API based on the **Builder** and **Decorator** patterns.

```rust
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
    // This example demonstrates 2 scenarios where it proceeds without any hurdles
    // and it impedes by a failure. 
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