use crate::relaxed_r1cs::{CommittedRelaxedR1CS, RelaxedR1CS};

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

    pub(crate) fn commit(&self, m: &Vec<C::Scalar>, r: C::Scalar) -> C {
        (self.h * r
            + m.iter()
                .zip(self.domain.iter())
                .fold(C::Extended::ADDITIVE_IDENTITY, |sum, (v, e)| sum + *e * *v))
        .into()
    }

    pub(crate) fn commit_relaxed_r1cs(
        &self,
        relaxed_r1cs: &RelaxedR1CS<C::Scalar>,
        w: &Vec<C::Scalar>,
        x: &Vec<C::Scalar>,
        cs: &CommitmentScheme<C>,
    ) -> CommittedRelaxedR1CS<C> {
        CommittedRelaxedR1CS {
            overline_e: cs.commit(&relaxed_r1cs.e, relaxed_r1cs.u),
            u: relaxed_r1cs.u,
            overline_w: cs.commit(&w, relaxed_r1cs.u),
            x: x.to_vec(),
        }
    }
}
