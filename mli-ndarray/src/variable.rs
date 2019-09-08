use mli::{Backward, Forward, Train};
use ndarray::{Array, Dimension};
use std::ops::AddAssign;

/// This wraps a D-dimesnional ndarray that acts as a constant input in a neural network.
/// It can be learned through training as well. It has no input.
pub struct Variable<T, D>(pub Array<T, D>);

impl<T, D> Forward for Variable<T, D>
where
    T: Clone,
    D: Clone,
{
    type Input = ();
    type Internal = ();
    type Output = Array<T, D>;

    fn forward(&self, (): &Self::Input) -> ((), Self::Output) {
        ((), self.0.clone())
    }
}

impl<T, D> Backward for Variable<T, D>
where
    T: Clone,
    D: Clone,
{
    type OutputDelta = Array<T, D>;
    type InputDelta = ();
    type TrainDelta = Array<T, D>;

    fn backward(
        &self,
        (): &Self::Input,
        (): &Self::Internal,
        output_delta: &Self::OutputDelta,
    ) -> (Self::InputDelta, Self::TrainDelta) {
        ((), output_delta.clone())
    }
}

impl<T, D> Train for Variable<T, D>
where
    T: Clone + AddAssign,
    D: Clone + Dimension,
{
    fn train(&mut self, train_delta: &Self::TrainDelta) {
        self.0 += train_delta;
    }
}
