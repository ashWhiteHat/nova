use crate::matrix::DenseVectors;
use crate::r1cs::R1csStructure;

use zkstd::common::{Group, TwistedEdwardsAffine};

#[derive(Clone, Debug)]
pub struct Witness<C: TwistedEdwardsAffine> {
    /// witness
    pub(crate) w: DenseVectors<C::Scalar>,
    /// error vectors
    pub(crate) e: DenseVectors<C::Scalar>,
}

impl<C: TwistedEdwardsAffine> Witness<C> {
    pub(crate) fn get(&self) -> (DenseVectors<C::Scalar>, DenseVectors<C::Scalar>) {
        (self.e.clone(), self.w.clone())
    }

    pub(crate) fn init(r1cs: R1csStructure<C>) -> Self {
        Self {
            w: DenseVectors(vec![C::Scalar::zero(); r1cs.m - r1cs.l]),
            e: DenseVectors(vec![C::Scalar::zero(); r1cs.l]),
        }
    }
}
