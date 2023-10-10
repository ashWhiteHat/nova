use crate::hash::Digest;
use crate::matrix::DenseVectors;

use zkstd::common::{CurveAffine, CurveGroup, RngCore};

pub(crate) struct Proof<C: CurveAffine> {
    r: C,
    u: Vec<C::Scalar>,
    r_u: C::Scalar,
}

pub struct PedersenCommitment<C: CurveAffine> {
    h: C,
    g: Vec<C>,
}

impl<C: CurveAffine> PedersenCommitment<C> {
    pub(crate) fn new(n: u64, mut r: impl RngCore) -> Self {
        let h = C::Affine::random(&mut r).into();
        let g = (0..=1 << n)
            .map(|_| C::Affine::random(&mut r).into())
            .collect();
        Self { h, g }
    }

    pub(crate) fn commit(&self, m: &DenseVectors<C::Scalar>, r: &C::Scalar) -> C {
        (self.h * r
            + m.iter()
                .zip(self.g.iter())
                .fold(C::Extended::ADDITIVE_IDENTITY, |sum, (v, e)| sum + *e * v))
        .into()
    }

    pub(crate) fn digest(&self) -> C::Scalar {
        let mut hasher = Digest::default();
        self.g
            .iter()
            .for_each(|base| hasher.update(&base.to_raw_bytes()));
        hasher.finalize()
    }

    pub(crate) fn size(&self) -> usize {
        self.g.len()
    }
}
