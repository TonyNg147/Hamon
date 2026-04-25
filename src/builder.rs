use std::marker::PhantomData;

use crate::{
    step::{FirstStep, GuardedStep, Step},
    utils::FirstStepIndex,
    Decorator,
};

/// The entry point for creating a new transformation pipeline.
pub struct Builder<T> {
    value: T,
}

/// Under certain scenarios, when the order matters. Some steps cannot be performed unless
/// others have completed or
///
/// This builder shines when the arbitrary appearance of Step must be certain at some points
pub struct OrderedBuild<T> {
    value: T,
}

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

impl<T> OrderedBuild<T> {
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
