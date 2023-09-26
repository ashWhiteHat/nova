use crate::{committed_relaxed_r1cs::CommittedRelaxedR1csStructure, matrix::SparseMatrix};

use zkstd::common::PrimeField;

#[derive(Clone, Debug)]
pub(crate) struct RelaxedR1csStructure<F: PrimeField> {
    /// instance length
    pub(crate) m: usize,
    pub(crate) l: usize,
    pub(crate) a: SparseMatrix<F>,
    pub(crate) b: SparseMatrix<F>,
    pub(crate) c: SparseMatrix<F>,
}

impl<F: PrimeField> RelaxedR1csStructure<F> {
    pub(crate) fn commit(&self) -> CommittedRelaxedR1csStructure<F> {
        let Self { m, l, a, b, c } = self.clone();
        CommittedRelaxedR1csStructure { m, l, a, b, c }
    }
}
