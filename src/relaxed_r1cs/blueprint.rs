use crate::matrix::{DenseVectors, SparseMatrix};

use zkstd::common::PrimeField;

pub(crate) struct RelaxedR1CS<F: PrimeField> {
    /// error vectors
    pub(crate) e: DenseVectors<F>,
    /// scalar
    pub(crate) u: F,
    /// instance length
    pub(crate) l: usize,
    pub(crate) a: SparseMatrix<F>,
    pub(crate) b: SparseMatrix<F>,
    pub(crate) c: SparseMatrix<F>,
}
