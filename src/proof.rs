use crate::r1cs::{Instance as R1csInstance, Witness as R1csWitness};
use crate::relaxed_r1cs::{Instance as RelaxedR1csInstance, Witness as RelaxedR1csWitness};

use zkstd::common::TwistedEdwardsAffine;

pub(crate) struct IvcProof<C: TwistedEdwardsAffine> {
    pub(crate) upper_pair: (RelaxedR1csInstance<C>, RelaxedR1csWitness<C>),
    pub(crate) lower_pair: (R1csInstance<C::Scalar>, R1csWitness<C>),
}
