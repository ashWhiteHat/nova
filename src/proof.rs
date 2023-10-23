use crate::r1cs::{Instance as R1csInstance, Witness as R1csWitness};
use crate::relaxed_r1cs::{Instance as RelaxedR1csInstance, Witness as RelaxedR1csWitness};

use zkstd::common::CurveAffine;

pub(crate) struct IvcProof<C: CurveAffine> {
    pub(crate) upper_pair: (
        RelaxedR1csInstance<C::Scalar>,
        RelaxedR1csWitness<C::Scalar>,
    ),
    pub(crate) lower_pair: (R1csInstance<C::Scalar>, R1csWitness<C::Scalar>),
}
