use zkstd::common::{CurveAffine, CurveGroup, RngCore};

pub(crate) struct Proof<C: CurveAffine> {
    r: C,
    u: Vec<C::Scalar>,
    r_u: C::Scalar,
}

pub(crate) struct CommitmentScheme<C: CurveAffine> {
    h: C,
    domain: Vec<C>,
}

impl<C: CurveAffine> CommitmentScheme<C> {
    pub(crate) fn new(k: u64, mut r: impl RngCore) -> Self {
        let h = C::Affine::random(&mut r).into();
        let domain = (0..=1 << k)
            .map(|_| C::Affine::random(&mut r).into())
            .collect();
        Self { h, domain }
    }

    pub(crate) fn commit(&self, m: Vec<C::Scalar>, r: C::Scalar) -> C {
        (self.h * r
            + m.iter()
                .zip(self.domain.iter())
                .fold(C::Extended::ADDITIVE_IDENTITY, |sum, (v, e)| sum + *e * *v))
        .into()
    }
}