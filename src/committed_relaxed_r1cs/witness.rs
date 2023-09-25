use crate::matrix::DenseVectors;

use zkstd::common::PrimeField;

/// witness for committed relaxed r1cs
#[derive(Debug)]
pub struct CommittedRelaxedR1csWitness<F: PrimeField> {
    /// error vectors
    pub(crate) e: DenseVectors<F>,
    /// commitment randomness for E
    pub(crate) r_e: F,
    /// intermediate value and private inputs
    pub(crate) w: DenseVectors<F>,
    /// commitment randomness for W
    pub(crate) r_w: F,
}
