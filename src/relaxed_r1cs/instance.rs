use crate::matrix::DenseVectors;

use zkstd::common::PrimeField;

/// instance for relaxed r1cs (E, u, x)
#[derive(Clone, Debug)]
pub struct RelaxedR1csInstanceData<F: PrimeField> {
    /// error vectors
    pub(crate) e: DenseVectors<F>,
    /// scalar
    pub(crate) u: F,
    /// public inputs and outputs
    pub(crate) x: DenseVectors<F>,
}

impl<F: PrimeField> RelaxedR1csInstanceData<F> {
    pub(crate) fn new(m: usize, x: DenseVectors<F>) -> Self {
        Self {
            e: DenseVectors(vec![F::zero(); m]),
            u: F::one(),
            x,
        }
    }
}
