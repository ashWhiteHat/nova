use crate::committed_relaxed_r1cs::Witness as CommittedRelaxedR1csWitness;
use crate::matrix::DenseVectors;
use crate::r1cs::R1csStructure;

use zkstd::common::PrimeField;

/// witness for relaxed r1cs Z = (W, x, u)
#[derive(Clone, Debug)]
pub struct Witness<F: PrimeField> {
    /// intermediate value and private inputs
    pub(crate) w: DenseVectors<F>,
    /// public inputs and outputs
    pub(crate) x: DenseVectors<F>,
    /// scalar
    pub(crate) u: F,
}

impl<F: PrimeField> Witness<F> {
    pub(crate) fn commit(
        &self,
        e: DenseVectors<F>,
        r_e: F,
        r_w: F,
    ) -> CommittedRelaxedR1csWitness<F> {
        let w: DenseVectors<F> = self.w.clone();
        CommittedRelaxedR1csWitness { e, r_e, w, r_w }
    }

    pub(crate) fn get(&self) -> (DenseVectors<F>, DenseVectors<F>) {
        (self.x.clone(), self.w.clone())
    }

    pub(crate) fn init(r1cs: R1csStructure<F>) -> Self {
        Self {
            w: DenseVectors(vec![F::zero(); r1cs.m - r1cs.l]),
            x: DenseVectors(vec![F::zero(); r1cs.l]),
            u: F::one(),
        }
    }
}
