use crate::matrix::DenseVectors;

use zkstd::common::PrimeField;

/// instance for relaxed r1cs (E, u, x)
#[derive(Clone, Debug)]
pub struct Instance<F: PrimeField> {
    /// error vectors
    pub(crate) e: DenseVectors<F>,
    /// scalar
    pub(crate) u: F,
    /// public inputs and outputs
    pub(crate) x: DenseVectors<F>,
}
