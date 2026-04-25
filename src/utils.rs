use crate::errors::Result;
use crate::Decorator;
use std::marker::PhantomData;

pub trait StepIndexBase {
    const ID: usize;
}

pub struct FirstStepIndex;

impl StepIndexBase for FirstStepIndex {
    const ID: usize = 1;
}

pub struct StepIndex<T>(pub(crate) PhantomData<T>);

impl<T> StepIndexBase for StepIndex<T>
where
    T: StepIndexBase,
{
    const ID: usize = T::ID + 1;
}

/// Trait marker to allow the transition from one step to another.
///
/// Implementations of this Trait on some Steps presents the permit for smooth transition.
pub trait FromStep<T> {}

/// Marker trait for decorators that can be preceeded by ANY state.
pub trait AnyStep {}

impl<D, T> FromStep<D> for T where T: AnyStep {}

/// Blanket implementation for any closure that matches the signature.
/// This allows for instant, flexible strikes without defining a new struct.
impl<T, O, F> Decorator<T, O> for F
where
    F: FnMut(T) -> Result<O>,
{
    #[inline]
    fn produce(&mut self, previous: T) -> Result<O> {
        self(previous)
    }
}
