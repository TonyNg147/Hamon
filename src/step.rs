use crate::errors::Result;
use crate::{
    utils::{FromStep, StepIndex, StepIndexBase},
    Collector, Decorator,
};
use std::marker::PhantomData;

pub struct FirstStep<T>(pub(crate) T);

impl<T> Collector<T> for FirstStep<T> {
    fn collect(self) -> Result<T> {
        Ok(self.0)
    }
}

/// Represent an active stage in the transformation pipeline.
#[must_use = "Steps do nothing unless you consume (collect) them"]
pub struct Step<D, P, T, ID> {
    pub(crate) decorator: D,
    pub(crate) previous: P,
    pub(crate) _previous_type: PhantomData<T>,
    pub(crate) _index: ID,
}

impl<D, P, T, ID> Step<D, P, T, ID> {
    /// Applies a transformation to the current value and potentially changes existing type
    ///
    /// However, the transformation is lazy which means it only takes effect after you consume [`Step`] value.
    /// Typical `step` registrations would be simply stacked up to produce a chain of step pipeline.
    pub fn step<NewD, T1, T2>(self, decorator: NewD) -> Step<NewD, Self, T1, StepIndex<ID>>
    where
        NewD: Decorator<T1, T2>,
        D: Decorator<T, T1>,
    {
        Step {
            decorator,
            previous: self,
            _previous_type: PhantomData,
            _index: StepIndex(PhantomData),
        }
    }

    /// Provide the depth for the current `step`. At any time you can query where the current step's at.
    pub fn get_index(&self) -> usize
    where
        ID: StepIndexBase,
    {
        ID::ID
    }
}

impl<D, P, T, ID, O> Collector<O> for Step<D, P, T, ID>
where
    D: Decorator<T, O>,
    P: Collector<T>,
{
    /// Finalizes the pipeline and returns the resulting value.
    fn collect(mut self) -> Result<O> {
        self.decorator.produce(self.previous.collect()?)
    }
}

/// Unlike the typical step, this special is equipped with the ability to detect any up-front constraints
///
/// With respect to Rust Philosophy "Make illegal states unrepresentable", by bounding we can ensure only eligible states (STEPS)
/// being instantiated.
pub struct GuardedStep<D, P, T, ID> {
    pub(crate) decorator: D,
    pub(crate) previous: P,
    pub(crate) _previous_type: PhantomData<T>,
    pub(crate) _index: ID,
}

impl<D, P, T, ID> GuardedStep<D, P, T, ID> {
    /// Applies a transformation to the current value and potentially changes existing type
    ///
    /// However, the transformation is lazy which means it only takes effect after you consume [`Step`] value.
    /// Typical `step` registrations would be simply stacked up to produce a chain of step pipeline.
    pub fn step<NewD, T1, T2>(self, decorator: NewD) -> GuardedStep<NewD, Self, T1, StepIndex<ID>>
    where
        NewD: Decorator<T1, T2> + FromStep<D>,
        D: Decorator<T, T1>,
    {
        // Here we creata a type bound. From the Transition from D -> NewD
        // The type NewD must provide the implemenation for trait marker FromStep with the
        // particular type of D
        // By that way the strong bond will be enforced.
        GuardedStep {
            decorator,
            previous: self,
            _previous_type: PhantomData,
            _index: StepIndex(PhantomData),
        }
    }

    /// Provide the depth for the current `step`. At any time you can query where the current step's at.
    pub fn get_index(&self) -> usize
    where
        ID: StepIndexBase,
    {
        ID::ID
    }
}

impl<D, P, T, ID, O> Collector<O> for GuardedStep<D, P, T, ID>
where
    D: Decorator<T, O>,
    P: Collector<T>,
{
    /// Finalizes the pipeline and returns the resulting value.
    fn collect(mut self) -> Result<O> {
        self.decorator.produce(self.previous.collect()?)
    }
}
