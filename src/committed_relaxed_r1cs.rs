mod blueprint;
mod witness;

pub(crate) use blueprint::CommittedRelaxedR1cs;
pub(crate) use witness::CommittedRelaxedR1csWitness;
use zkstd::common::CurveAffine;

pub(crate) struct CommittedRelaxedR1csInstance<C: CurveAffine> {
    pub(crate) committed_relaxed_r1cs: CommittedRelaxedR1cs<C>,
    pub(crate) committed_relaxed_z: CommittedRelaxedR1csWitness<C::Scalar>,
}
