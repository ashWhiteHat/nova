use crate::matrix::DenseVectors;

use zkstd::common::PrimeField;

/// instance for r1cs x
#[derive(Debug, Default)]
pub struct Instance<F: PrimeField> {
    /// public inputs and outputs
    pub(crate) x: DenseVectors<F>,
}
