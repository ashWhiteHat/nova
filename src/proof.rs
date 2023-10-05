use crate::committed_relaxed_r1cs::CommittedRelaxedR1csInstance;
use crate::r1cs::Witness as R1csWitness;

use zkstd::common::CurveAffine;

pub(crate) struct IvcProof<C: CurveAffine> {
    pub(crate) upper_pair: (CommittedRelaxedR1csInstance<C>, R1csWitness<C::Scalar>),
    pub(crate) lower_pair: (CommittedRelaxedR1csInstance<C>, R1csWitness<C::Scalar>),
}
