use crate::matrix::DenseVectors;

use zkstd::common::CurveAffine;

pub(crate) struct CommittedRelaxedR1CS<C: CurveAffine> {
    pub(crate) overline_e: C,
    pub(crate) u: C::Scalar,
    pub(crate) overline_w: C,
    pub(crate) x: DenseVectors<C::Scalar>,
}
