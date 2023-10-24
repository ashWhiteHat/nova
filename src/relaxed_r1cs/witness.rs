use crate::matrix::DenseVectors;
use crate::r1cs::{R1csStructure, Witness as R1csWitness};

use zkstd::common::{Group, PrimeField, TwistedEdwardsAffine};

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
            e: DenseVectors(vec![C::Scalar::zero(); r1cs.m]),
        }
    }

    pub(crate) fn fold(
        &self,
        witness: R1csWitness<C>,
        r: C::Scalar,
        t: DenseVectors<C::Scalar>,
    ) -> Self {
        let r2 = r.square();
        let e2 = self.e.clone();
        let w1 = witness.w;
        let w2 = self.w.clone();

        Self {
            e: t * r + e2 * r2,
            w: w1 + w2 * r,
        }
    }
}
