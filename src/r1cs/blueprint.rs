use crate::matrix::{DenseVectors, Element, SparseMatrix};
use crate::relaxed_r1cs::RelaxedR1csStructure;

use zkstd::common::PrimeField;

pub(crate) use super::instance::Instance;
pub(crate) use super::witness::Witness;
use super::R1csInstance;

/// https://eprint.iacr.org/2021/370.pdf
/// 4.1 Definition 10 R1CS
///  (A · Z) ◦ (B · Z) = C · Z
#[derive(Clone, Debug)]
pub struct R1csStructure<F: PrimeField> {
    /// matrix length
    pub(crate) m: usize,
    /// instance length
    pub(crate) l: usize,
    pub(crate) a: SparseMatrix<F>,
    pub(crate) b: SparseMatrix<F>,
    pub(crate) c: SparseMatrix<F>,
}

impl<F: PrimeField> Default for R1csStructure<F> {
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

impl<F: PrimeField> R1csStructure<F> {
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

    pub(crate) fn instantiate(&self, z: &Vec<F>) -> R1csInstance<F> {
        let (instance, witness) = self.instance_and_witness(z);
        R1csInstance {
            r1cs: self.clone(),
            instance,
            witness,
        }
    }

    pub(crate) fn instance_and_witness(&self, witnesses: &Vec<F>) -> (Instance<F>, Witness<F>) {
        let w = DenseVectors(witnesses[self.l..].to_vec());
        let x = DenseVectors(witnesses[..self.l].to_vec());
        let one = F::one();
        (Instance { x: x.clone() }, Witness { w, x, one })
    }

    pub(crate) fn relax(&self) -> RelaxedR1csStructure<F> {
        let Self { m, l, a, b, c } = self.clone();
        RelaxedR1csStructure { m, l, a, b, c }
    }
}
