use crate::matrix::DenseVectors;
use crate::relaxed_r1cs::{CommittedRelaxedR1CS, RelaxedR1CSInstance, RelaxedR1csWitness};

use zkstd::common::{CurveAffine, CurveGroup, RngCore};

pub(crate) struct Proof<C: CurveAffine> {
    r: C,
    u: Vec<C::Scalar>,
    r_u: C::Scalar,
}

pub struct CommitmentScheme<C: CurveAffine> {
    h: C,
    domain: Vec<C>,
}

impl<C: CurveAffine> CommitmentScheme<C> {
    pub(crate) fn new(n: u64, mut r: impl RngCore) -> Self {
        let h = C::Affine::random(&mut r).into();
        let domain = (0..=1 << n)
            .map(|_| C::Affine::random(&mut r).into())
            .collect();
        Self { h, domain }
    }

    pub(crate) fn commit(&self, m: &DenseVectors<C::Scalar>, r: C::Scalar) -> C {
        (self.h * r
            + m.iter()
                .zip(self.domain.iter())
                .fold(C::Extended::ADDITIVE_IDENTITY, |sum, (v, e)| sum + *e * v))
        .into()
    }

    pub(crate) fn commit_relaxed_r1cs_instance(
        &self,
        relaxed_r1cs_instance: &RelaxedR1CSInstance<C::Scalar>,
    ) -> CommittedRelaxedR1CS<C> {
        let RelaxedR1CSInstance {
            relaxed_r1cs,
            relaxed_z,
        } = relaxed_r1cs_instance;
        let RelaxedR1csWitness { x, w, u } = relaxed_z;
        CommittedRelaxedR1CS {
            overline_e: self.commit(&relaxed_r1cs.e, *u),
            u: *u,
            overline_w: self.commit(w, *u),
            x: x.clone(),
        }
    }
}
