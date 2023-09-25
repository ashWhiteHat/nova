use crate::commitment::CommitmentScheme;
use crate::{
    committed_relaxed_r1cs::CommittedRelaxedR1CS,
    matrix::{DenseVectors, SparseMatrix},
};

use zkstd::common::{CurveAffine, PrimeField};

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

pub(crate) fn commit_relaxed_r1cs<C: CurveAffine>(
    relaxed_r1cs: &RelaxedR1CS<C::Scalar>,
    x: &DenseVectors<C::Scalar>,
    w: &DenseVectors<C::Scalar>,
    cs: &CommitmentScheme<C>,
) -> CommittedRelaxedR1CS<C> {
    CommittedRelaxedR1CS {
        overline_e: cs.commit(&relaxed_r1cs.e, &relaxed_r1cs.u),
        u: relaxed_r1cs.u,
        overline_w: cs.commit(w, &relaxed_r1cs.u),
        x: x.clone(),
    }
}
