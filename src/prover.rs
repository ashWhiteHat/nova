use crate::matrix::SparseMatrix;
use crate::public_param::PublicParams;

use zkstd::common::CurveAffine;

pub(crate) struct ProvingKey<C: CurveAffine> {
    pub(crate) pp: PublicParams<C>,
    pub(crate) a: SparseMatrix<C::Scalar>,
    pub(crate) b: SparseMatrix<C::Scalar>,
    pub(crate) c: SparseMatrix<C::Scalar>,
}
