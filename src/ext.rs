//! # Pipeline Extensions
//!
//! Optional enhancements that add runtime flexibility to static pipelines.
//!
//! ## Conditional Decorators
//! The `DecoratorExt` trait provides the `.when()` method, allowing decorators
//! to be conditionally applied based on input predicates. This maintains the
//! zero-cost property - conditions are evaluated inline without dynamic dispatch.
//!
//! ## Usage
//! ```rust
//! expensive_decorator.when(|input| input.needs_processing())
//! ```
//!
//! The condition check becomes part of the generated machine code, with no
//! additional abstraction layers.
use crate::errors::Result;
use crate::Decorator;

pub struct ConditionDecorator<D, P> {
    decorator: D,
    pred: P,
}

impl<T, D, P> Decorator<T, T> for ConditionDecorator<D, P>
where
    D: Decorator<T, T>,
    P: FnMut(&T) -> bool,
{
    fn produce(&mut self, input: T) -> Result<T> {
        if (self.pred)(&input) {
            self.decorator.produce(input)
        } else {
            Ok(input)
        }
    }
}

/// A trait extends the optional validation on the decorator being declared
///
/// It's the extension so the usage of it falls entirely on your hands. Once, you have imported this trait
/// any registered decorators can only take effect if the condition applies to it evaluated to True
pub trait DecoratorExt<T>: Decorator<T, T> {
    /// Combine the condition with the current decorator. Later when it's being consumed via `step` or `build`
    /// The determination of wrap up with the existing decorator is the result of the Predicate.
    fn when<P>(self, pred: P) -> ConditionDecorator<Self, P>
    where
        P: FnMut(&T) -> bool,
        Self: Sized,
    {
        ConditionDecorator {
            decorator: self,
            pred,
        }
    }
}

impl<S, T> DecoratorExt<T> for S where S: Decorator<T, T> {}
