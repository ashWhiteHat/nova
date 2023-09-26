use crate::committed_relaxed_r1cs::CommittedRelaxedR1csWitness;
use crate::matrix::DenseVectors;

use zkstd::common::PrimeField;

/// witness for relaxed r1cs Z = (W, x, u)
#[derive(Clone, Debug)]
pub struct RelaxedR1csWitness<F: PrimeField> {
    /// intermediate value and private inputs
    pub(crate) w: DenseVectors<F>,
    /// public inputs and outputs
    pub(crate) x: DenseVectors<F>,
    /// scalar
    pub(crate) u: F,
}

pub(crate) fn commit_relaxed_witness<F: PrimeField>(
    relaxed_r1cs_witness: &RelaxedR1csWitness<F>,
    e: DenseVectors<F>,
    r_e: F,
    r_w: F,
) -> CommittedRelaxedR1csWitness<F> {
    let w = relaxed_r1cs_witness.w.clone();
    CommittedRelaxedR1csWitness { e, r_e, w, r_w }
}
