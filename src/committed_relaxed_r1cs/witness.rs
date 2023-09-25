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

impl<F: PrimeField> CommittedRelaxedR1csWitness<F> {
    pub(crate) fn new(e: DenseVectors<F>, r_e: F, w: DenseVectors<F>, r_w: F) -> Self {
        Self { e, r_e, w, r_w }
    }

    pub(crate) fn get(&self) -> (DenseVectors<F>, F, DenseVectors<F>, F) {
        (self.e.clone(), self.r_e, self.w.clone(), self.r_w)
    }
}
