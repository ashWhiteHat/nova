use crate::matrix::DenseVectors;

use zkstd::common::PrimeField;

/// witness for r1cs
#[derive(Debug)]
pub struct RelaxedR1csWitness<F: PrimeField> {
    /// public inputs and outputs
    pub(crate) x: DenseVectors<F>,
    /// intermediate value and private inputs
    pub(crate) w: DenseVectors<F>,
    /// scalar
    pub(crate) u: F,
}
