use crate::matrix::DenseVectors;
use crate::r1cs::{Instance as R1csInstance, R1csStructure};

use zkstd::common::{Group, PrimeField, Ring, TwistedEdwardsAffine};

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

    pub(crate) fn fold(&self, instance: R1csInstance<C::Scalar>, r: C::Scalar, t: C) -> Self {
        let r2 = r.square();
        let e1 = C::ADDITIVE_IDENTITY;
        let e2 = self.commit_e;
        let u1 = C::Scalar::one();
        let u2 = self.u;
        let w1 = C::ADDITIVE_IDENTITY;
        let w2 = self.commit_w;
        let x1 = instance.x;
        let x2 = self.x.clone();

        Self {
            commit_e: (e1 + t * r + e2 * r2).into(),
            u: u1 + r * u2,
            commit_w: (w1 + w2 * r).into(),
            x: x1 + x2 * r,
        }
    }
}
