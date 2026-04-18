//! # Hamon
//!
//! `hamon` is a high-performance, zero-cost static decorator library.
//!
//! ## Core Philosophy
//! This crate provides a way to compose data processing pipelines that are
//! resolved at compile-time. By leveraging Rust's type system, it eliminates
//! the overhead of dynamic dispatch (`dyn`) while maintaining modularity.
//!
//! ## Quick Start
//! ```rust
//! use hamon::prelude::*;
//!
//! let result = Builder::new(10)
//!     .step(|x| x + 5)
//!     .build();
//! ```
//!
pub mod builder;
pub mod ext;
pub mod prelude;

pub use hamon_derive::AllowStep;

/// A trait for types that can transform an input `T` into an output `O`.
///
/// In the Hamon philosophy, this is the 'Edge'—the logic that defines
/// how data is tempered as it passes through the pipeline.
pub trait Decorator<T, O> {
    /// Consumes or mutates the decorator to produce a result from the input.
    fn produce(&mut self, input: T) -> O;
}

/// Blanket implementation for any closure that matches the signature.
/// This allows for instant, flexible strikes without defining a new struct.
impl<T, O, F> Decorator<T, O> for F
where
    F: FnMut(T) -> O,
{
    #[inline]
    fn produce(&mut self, previous: T) -> O {
        self(previous)
    }
}
