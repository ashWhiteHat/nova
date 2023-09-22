use crate::matrix::SparseMatrix;

use zkstd::common::{CurveAffine, PrimeField};

pub(crate) struct RelaxedR1CS<F: PrimeField> {
    /// error vectors
    pub(crate) e: Vec<F>,
    /// scalar
    pub(crate) u: F,
    /// instance length
    pub(crate) l: usize,
    pub(crate) a: SparseMatrix<F>,
    pub(crate) b: SparseMatrix<F>,
    pub(crate) c: SparseMatrix<F>,
}

pub(crate) struct CommittedRelaxedR1CS<C: CurveAffine> {
    pub(crate) overline_e: C,
    pub(crate) u: C::Scalar,
    pub(crate) overline_w: C,
    pub(crate) x: Vec<C::Scalar>,
}
