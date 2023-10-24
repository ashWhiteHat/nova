use crate::matrix::DenseVectors;
use crate::r1cs::R1csStructure;

use zkstd::common::{Group, Ring, TwistedEdwardsAffine};

/// instance for relaxed r1cs (E, u, x)
#[derive(Clone, Debug)]
pub struct Instance<C: TwistedEdwardsAffine> {
    /// commitment for witness vectors
    pub(crate) commit_w: C,
    /// commitment for error vectors
    pub(crate) commit_e: C,
    /// scalar
    pub(crate) u: C::Scalar,
    /// public inputs and outputs
    pub(crate) x: DenseVectors<C::Scalar>,
}

impl<C: TwistedEdwardsAffine> Instance<C> {
    pub(crate) fn init(r1cs: R1csStructure<C>) -> Self {
        Self {
            commit_w: C::ADDITIVE_IDENTITY,
            commit_e: C::ADDITIVE_IDENTITY,
            u: C::Scalar::one(),
            x: DenseVectors(vec![C::Scalar::zero(); r1cs.l]),
        }
    }
}
