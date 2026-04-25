//! # Hamon (刃文) - Zero-Cost Static Decorators
//!
//! A high-performance library that enables composing data processing pipelines
//! resolved entirely at compile time through Rust's type system.
//!
//! ## Philosophy
//! Traditional decorator patterns sacrifice performance for modularity through
//! dynamic dispatch. Hamon achieves both by leveraging monomorphization -
//! the compiler generates specialized machine code for each pipeline,
//! eliminating indirection while preserving clean, composable APIs.
//!
//! ## Performance Model
//! - **Compile Time**: Generic recursion builds type-level pipeline structure
//! - **Runtime**: Direct function calls with zero abstraction overhead
//! - **Memory**: Stack-first design avoids heap fragmentation
//!
//! ## Core Abstraction
//! The `Decorator<T, O>` trait represents a transformation edge in the pipeline.
//! Through recursive generics, complex chains become nested types that LLVM
//! can optimize into flat, efficient assembly.
pub mod builder;
pub mod errors;
pub mod ext;
pub mod prelude;
pub mod step;
pub mod utils;

pub use hamon_derive::AllowStep;

/// A trait for types that can transform an input `T` into an output `O`.
///
/// In the Hamon philosophy, this is the 'Edge'—the logic that defines
/// how data is tempered as it passes through the pipeline.
pub trait Decorator<T, O> {
    /// Consumes or mutates the decorator to produce a result from the input.
    fn produce(&mut self, input: T) -> errors::Result<O>;
}

/// This trait imposes the standard behavior which Steps should follow.
///
/// Upon invoking, it would trigger the collection by processing any piled up Decorator.
pub trait Collector<T> {
    fn collect(self) -> errors::Result<T>;
}
