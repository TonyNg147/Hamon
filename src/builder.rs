//! Dedicated to collecting the pipelines output
//!
//! Speaking of *pipeline*, *Builder* is an entrypoint opening the door for processing data through pipeline
//! Once specifying all the necessary, we can consume the to get the final look of initial data via `.collect()`
//!
//! The `builder` module brings forth 2 kinds of Builder
//! - `Builder`: for the trivial collection of transformation where each step doesn't compel particular relation
//! - `GuardedBuilder`: comes in handy if ORDER is matter. Steps allow any arbitrary steps or a specific one preceding it.
use std::marker::PhantomData;

use crate::{
    step::{FirstStep, GuardedStep, Step},
    utils::FirstStepIndex,
    Decorator,
};

/// The entry point for creating a new transformation pipeline.
///
/// ```rust
/// use hamon::errors::Result;
/// use hamon::prelude::*;
/// struct Add(i32);
///
/// impl Decorator<i32, i32> for Add {
///     fn produce(&mut self, input: i32) -> Result<i32> {
///         println!("{:<10}: previous value was {}", "[ADD]", input);
///         Ok(self.0 + input)
///     }
/// }
///
/// let engine = Builder::new(10)
///                     .step(Add(2))
///                     .collect(); // 12
/// ```
pub struct Builder<T> {
    value: T,
}

/// Serves the same purpose as [`Builder`] except adding an assurance for type registration
///
/// Under certain scenarios, when the order matters. Some steps cannot be performed unless
/// others have completed.
///
/// This builder shines when the arbitrary appearance of Step must be certain at some points
/// ```rust
/// struct Encyption;
/// #[derive(AllowStep)]
/// #[from(Encyption)]
/// struct Compression;
///
/// let _result = OrderedBuilder::new(10)
///                 .step(Encyption)
///                 .step(Compression)
///                 .collect();
/// ```
pub struct OrderedBuilder<T> {
    value: T,
}

/// Implementation block for Typical Builder
impl<T> Builder<T> {
    /// Creates a new builder with an initial value.
    pub fn new(initial: T) -> Self {
        Builder { value: initial }
    }

    /// Initiates the first step of the pipeline.
    pub fn step<NewD, O>(self, decorator: NewD) -> Step<NewD, FirstStep<T>, T, FirstStepIndex>
    where
        NewD: Decorator<T, O>,
    {
        Step {
            decorator,
            previous: FirstStep(self.value),
            _previous_type: PhantomData,
            _index: FirstStepIndex,
        }
    }
}

/// Implementation block for OrderedBuilderer
impl<T> OrderedBuilder<T> {
    /// Creates a new OrderBuilder with an initial value.
    pub fn new(initial: T) -> Self {
        Self { value: initial }
    }

    pub fn step<NewD, O>(
        self,
        decorator: NewD,
    ) -> GuardedStep<NewD, FirstStep<T>, T, FirstStepIndex>
    where
        NewD: Decorator<T, O>,
    {
        GuardedStep {
            decorator,
            previous: FirstStep(self.value),
            _previous_type: PhantomData,
            _index: FirstStepIndex,
        }
    }
}
