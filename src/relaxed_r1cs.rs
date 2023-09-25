use crate::matrix::{DenseVectors, SparseMatrix};
use crate::r1cs::R1csWitness;

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

impl<F: PrimeField> RelaxedR1CS<F> {
    pub(crate) fn to_instance(&self, witness: &R1csWitness<F>) -> RelaxedR1CSInstance<F> {
        RelaxedR1CSInstance {
            e: self.e.clone(),
            u: self.u.clone(),
            x: witness.x.clone(),
            w: witness.w.clone(),
        }
    }
}

pub(crate) struct RelaxedR1CSInstance<F: PrimeField> {
    /// error vectors
    pub(crate) e: DenseVectors<F>,
    /// scalar
    pub(crate) u: F,
    /// public inputs and outputs
    pub(crate) x: DenseVectors<F>,
    /// witness
    pub(crate) w: DenseVectors<F>,
}

pub(crate) struct CommittedRelaxedR1CS<C: CurveAffine> {
    pub(crate) overline_e: C,
    pub(crate) u: C::Scalar,
    pub(crate) overline_w: C,
    pub(crate) x: DenseVectors<C::Scalar>,
}
