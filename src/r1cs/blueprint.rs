use crate::matrix::{DenseVectors, Element, SparseMatrix};
use crate::relaxed_r1cs::RelaxedR1CS;

use zkstd::common::PrimeField;

pub(crate) use super::witness::R1csWitness;

/// https://eprint.iacr.org/2021/370.pdf
/// 4.1 Definition 10 R1CS
///  (A · Z) ◦ (B · Z) = C · Z
#[derive(Clone, Debug)]
pub struct R1cs<F: PrimeField> {
    /// matrix length
    pub(crate) m: usize,
    /// instance length
    pub(crate) l: usize,
    pub(crate) a: SparseMatrix<F>,
    pub(crate) b: SparseMatrix<F>,
    pub(crate) c: SparseMatrix<F>,
}

impl<F: PrimeField> Default for R1cs<F> {
    fn default() -> Self {
        Self {
            m: 0,
            l: 0,
            a: SparseMatrix(vec![vec![]]),
            b: SparseMatrix(vec![vec![]]),
            c: SparseMatrix(vec![vec![]]),
        }
    }
}

impl<F: PrimeField> R1cs<F> {
    pub(crate) fn append(
        &mut self,
        a: impl Into<Element<F>>,
        b: impl Into<Element<F>>,
        c: impl Into<Element<F>>,
    ) {
        self.a[self.m].push(a.into());
        self.b[self.m].push(b.into());
        self.c[self.m].push(c.into());
    }

    pub(crate) fn append_a(&mut self, a: impl Into<Element<F>>) {
        self.a[self.m].push(a.into())
    }

    pub(crate) fn increment(&mut self) {
        self.a.0.push(vec![]);
        self.b.0.push(vec![]);
        self.c.0.push(vec![]);
        self.m += 1
    }

    pub(crate) fn instance_and_witness(&self, witnesses: Vec<F>) -> R1csWitness<F> {
        R1csWitness::new(witnesses[..self.l].to_vec(), witnesses[self.l..].to_vec())
    }

    pub(crate) fn relax(&self) -> RelaxedR1CS<F> {
        let Self { m, l, a, b, c } = self.clone();
        RelaxedR1CS {
            e: DenseVectors(vec![F::zero(); m]),
            u: F::one(),
            l,
            a,
            b,
            c,
        }
    }
}
