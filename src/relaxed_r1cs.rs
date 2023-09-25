mod blueprint;
mod witness;

use crate::matrix::DenseVectors;

pub(crate) use blueprint::RelaxedR1CS;
pub(crate) use witness::RelaxedR1csWitness;
use zkstd::common::{CurveAffine, PrimeField};

pub(crate) struct RelaxedR1CSInstance<F: PrimeField> {
    pub(crate) relaxed_r1cs: RelaxedR1CS<F>,
    pub(crate) relaxed_z: RelaxedR1csWitness<F>,
}

pub(crate) struct CommittedRelaxedR1CS<C: CurveAffine> {
    pub(crate) overline_e: C,
    pub(crate) u: C::Scalar,
    pub(crate) overline_w: C,
    pub(crate) x: DenseVectors<C::Scalar>,
}
