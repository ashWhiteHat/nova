mod blueprint;
mod witness;

pub(crate) use blueprint::CommittedRelaxedR1CS;
pub(crate) use witness::CommittedRelaxedR1csWitness;
use zkstd::common::CurveAffine;

pub(crate) struct CommittedRelaxedR1CSInstance<C: CurveAffine> {
    pub(crate) committed_relaxed_r1cs: CommittedRelaxedR1CS<C>,
    pub(crate) committed_relaxed_z: CommittedRelaxedR1csWitness<C::Scalar>,
}
