use crate::matrix::SparseMatrix;

use zkstd::common::PrimeField;

#[derive(Clone, Debug)]
pub(crate) struct CommittedRelaxedR1csStructure<F: PrimeField> {
    /// instance length
    pub(crate) m: usize,
    pub(crate) l: usize,
    pub(crate) a: SparseMatrix<F>,
    pub(crate) b: SparseMatrix<F>,
    pub(crate) c: SparseMatrix<F>,
}
