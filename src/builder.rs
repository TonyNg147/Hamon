use crate::Decorator;

/// The entry point for creating a new transformation pipeline.
pub struct Builder<T> {
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
