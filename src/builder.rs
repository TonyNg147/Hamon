use std::marker::PhantomData;

use crate::Decorator;

/// The entry point for creating a new transformation pipeline.
pub struct Builder<T> {
    value: T,
}

/// Under certain scenarios, when the order matters. Some steps cannot be performed unless
/// others have completed.
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
    pub fn step<O>(self, mut decorator: impl Decorator<T, O>) -> Step<O> {
        Step {
            cur: decorator.produce(self.value),
        }
    }
}

impl<T> OrderedBuild<T> {
    pub fn new(initial: T) -> Self {
        Self { value: initial }
    }

    pub fn step<O, D>(self, mut decorator: D) -> GuardedStep<O, D>
    where
        D: Decorator<T, O>,
    {
        GuardedStep {
            cur: decorator.produce(self.value),
            phantom: PhantomData,
        }
    }
}

/// Represents an active stage in the transformation pipeline.
#[must_use = "Steps do nothing unless you continue the chain or call .build()"]
pub struct Step<T> {
    pub(crate) cur: T,
}

impl<T> Step<T> {
    /// Applies a transformation to the current value, potentially changing its type.
    /// This is the 'Monomorphized' heart of the pipeline.
    pub fn step<O>(self, mut decorator: impl Decorator<T, O>) -> Step<O> {
        Step {
            cur: decorator.produce(self.cur),
        }
    }

    /// Finalizes the pipeline and returns the resulting value.
    pub fn build(self) -> T {
        self.cur
    }
}

/// Trait marker to allow the transition from one step to another.
///
/// Implementations of this Trait on some Steps presents the permit for smooth transition.
pub trait FromStep<T> {}

/// Marker trait for decorators that can follow ANY state.
pub trait AnyStep {}

impl<D, T> FromStep<D> for T where T: AnyStep {}

/// Unlike the typical step, this special is equipped with the ability to detect any up-front constraints
///
/// With respect to Rust Philosophy "Make illegal states unrepresentable", by bounding we can ensure only eligible states (STEPS)
/// being instantiated.
pub struct GuardedStep<T, D> {
    cur: T,
    phantom: PhantomData<D>, // The Step doesn't need to hold the decorator but it still requires the "information" of it for validation process.
}

impl<T, D> GuardedStep<T, D> {
    pub fn step<O, NewD>(self, mut decortor: NewD) -> GuardedStep<O, NewD>
    where
        Self: Sized,
        NewD: Decorator<T, O> + FromStep<Self>,
    {
        // Here we creata a type bound. From the Transition from D -> NewD
        // The type NewD must provide the implemenation for trait marker FromStep with the
        // particular type of D
        // By that way the strong bond will be enforced.
        GuardedStep {
            cur: decortor.produce(self.cur),
            phantom: PhantomData,
        }
    }

    pub fn build(self) -> T {
        self.cur
    }
}
