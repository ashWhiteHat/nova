use crate::commitment::CommitmentScheme;
use crate::r1cs::R1csStructure;

use zkstd::common::{CurveAffine, RngCore};

struct Nifs<C: CurveAffine> {
    pp: CommitmentScheme<C>,
}

impl<C: CurveAffine> Nifs<C> {
    pub(crate) fn setup(r1cs: R1csStructure<C::Scalar>, r: impl RngCore) -> Self {
        let n = r1cs.m.next_power_of_two() as u64;
        let pp: CommitmentScheme<C> = CommitmentScheme::new(n, r);
        Self { pp }
    }
}
