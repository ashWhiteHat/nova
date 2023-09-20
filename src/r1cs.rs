use crate::wire::Wire;

use zkstd::common::PrimeField;

/// https://eprint.iacr.org/2021/370.pdf
/// 4.1 Definition 10 R1CS
///  (A · Z) ◦ (B · Z) = C · Z
#[derive(Debug)]
pub(crate) struct R1cs<F: PrimeField> {
    pub(crate) m: usize,
    pub(crate) a: SparseMatrix<F>,
    pub(crate) b: SparseMatrix<F>,
    pub(crate) c: SparseMatrix<F>,
}

impl<F: PrimeField> Default for R1cs<F> {
    fn default() -> Self {
        Self {
            m: 0,
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
        self.a.0[self.m].push(a.into());
        self.b.0[self.m].push(b.into());
        self.c.0[self.m].push(c.into());
    }

    pub(crate) fn append_a(&mut self, a: impl Into<Element<F>>) {
        self.a.0[self.m].push(a.into())
    }

    pub(crate) fn increment(&mut self) {
        self.a.0.push(vec![]);
        self.b.0.push(vec![]);
        self.c.0.push(vec![]);
        self.m += 1
    }
}

#[derive(Debug, Default)]
pub(crate) struct SparseMatrix<F: PrimeField>(pub(crate) Vec<Vec<Element<F>>>);

#[derive(Debug)]
pub struct Element<F: PrimeField>(pub(crate) Wire, pub(crate) F);

impl<F: PrimeField> From<Wire> for Element<F> {
    fn from(value: Wire) -> Self {
        Self(value, F::one())
    }
}

impl<F: PrimeField> From<F> for Element<F> {
    fn from(value: F) -> Self {
        Self(Wire::one(), value)
    }
}

#[cfg(test)]
mod tests {
    use super::R1cs;
    use crate::tests::{array_to_witnessess, dense_to_sparse, is_satisfy};

    use bls_12_381::Fr as Scalar;

    #[test]
    fn r1cs_test() {
        // R1CS for: x^3 + x + 5 = y
        // https://www.vitalik.ca/general/2016/12/10/qap.html
        let m = 4;
        let a = dense_to_sparse::<Scalar>(vec![
            vec![0, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 0],
            vec![5, 0, 0, 0, 0, 1],
        ]);
        let b = dense_to_sparse::<Scalar>(vec![
            vec![0, 1, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0],
        ]);
        let c = dense_to_sparse::<Scalar>(vec![
            vec![0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 0, 1],
            vec![0, 0, 1, 0, 0, 0],
        ]);
        let r1cs = R1cs { m, a, b, c };
        let z = array_to_witnessess(vec![1, 3, 35, 9, 27, 30]);
        assert!(is_satisfy(&r1cs, z))
    }
}
