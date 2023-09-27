use crate::matrix::DenseVectors;

use zkstd::common::CurveAffine;

#[derive(Clone, Debug)]
pub(crate) struct Instance<C: CurveAffine> {
    /// commitment for error vectors
    pub(crate) overline_e: C,
    /// scalar
    pub(crate) u: C::Scalar,
    /// commitment for witness vectors
    pub(crate) overline_w: C,
    /// public inputs and outputs
    pub(crate) x: DenseVectors<C::Scalar>,
}

impl<C: CurveAffine> Instance<C> {
    pub(crate) fn get(&self) -> (C, C::Scalar, C, DenseVectors<C::Scalar>) {
        (self.overline_e, self.u, self.overline_w, self.x.clone())
    }
}
