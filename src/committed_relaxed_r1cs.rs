mod blueprint;
mod instance;
mod witness;

pub(crate) use blueprint::CommittedRelaxedR1csStructure;
pub(crate) use instance::Instance;
pub(crate) use witness::Witness;
use zkstd::common::CurveAffine;

pub struct CommittedRelaxedR1csInstance<C: CurveAffine> {
    pub(crate) committed_relaxed_r1cs: CommittedRelaxedR1csStructure<C::Scalar>,
    pub(crate) instance: Instance<C>,
    pub(crate) witness: Witness<C::Scalar>,
}
