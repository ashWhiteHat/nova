use crate::matrix::{DenseVectors, Element, SparseMatrix};

use zkstd::common::{Ring, TwistedEdwardsAffine};

pub(crate) use super::instance::Instance;
pub(crate) use super::witness::Witness;
use super::R1csInstance;

/// https://eprint.iacr.org/2021/370.pdf
/// 4.1 Definition 10 R1CS
///  (A · Z) ◦ (B · Z) = C · Z
#[derive(Clone, Debug)]
pub struct R1csStructure<C: TwistedEdwardsAffine> {
    /// matrix length
    pub(crate) m: usize,
    /// instance length
    pub(crate) l: usize,
    pub(crate) a: SparseMatrix<C::Scalar>,
    pub(crate) b: SparseMatrix<C::Scalar>,
    pub(crate) c: SparseMatrix<C::Scalar>,
}

impl<C: TwistedEdwardsAffine> Default for R1csStructure<C> {
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

impl<C: TwistedEdwardsAffine> R1csStructure<C> {
    pub(crate) fn append(
        &mut self,
        a: impl Into<Element<C::Scalar>>,
        b: impl Into<Element<C::Scalar>>,
        c: impl Into<Element<C::Scalar>>,
    ) {
        self.a[self.m].push(a.into());
        self.b[self.m].push(b.into());
        self.c[self.m].push(c.into());
    }

    pub(crate) fn append_a(&mut self, a: impl Into<Element<C::Scalar>>) {
        self.a[self.m].push(a.into())
    }

    pub(crate) fn increment(&mut self) {
        self.a.0.push(vec![]);
        self.b.0.push(vec![]);
        self.c.0.push(vec![]);
        self.m += 1
    }

    pub(crate) fn instantiate(&self, z: &Vec<C::Scalar>) -> R1csInstance<C> {
        let (instance, witness) = self.instance_and_witness(z);
        R1csInstance {
            r1cs: self.clone(),
            instance,
            witness,
        }
    }

    pub(crate) fn instance_and_witness(
        &self,
        witnesses: &Vec<C::Scalar>,
    ) -> (Instance<C::Scalar>, Witness<C>) {
        let w = DenseVectors(witnesses[self.l..].to_vec());
        let x = DenseVectors(witnesses[..self.l].to_vec());
        let one = C::Scalar::one();
        (Instance { x: x.clone() }, Witness { w, x, one })
    }
}
