use crate::wire::Wire;

use zkstd::common::PrimeField;

/// https://eprint.iacr.org/2021/370.pdf
/// 4.1 Definition 10 R1CS
///  (A · Z) ◦ (B · Z) = C · Z
#[derive(Debug, Default)]
pub(crate) struct R1cs<F: PrimeField> {
    m: usize,
    a: SparseMatrix<F>,
    b: SparseMatrix<F>,
    c: SparseMatrix<F>,
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

    pub(crate) fn increment(&mut self) {
        self.m += 1
    }
}

#[derive(Debug, Default)]
pub(crate) struct SparseMatrix<F: PrimeField>(Vec<Vec<Element<F>>>);

impl<F: PrimeField> SparseMatrix<F> {
    fn new(value: Vec<Vec<F>>) -> Self {
        let mut sparse_matrix = vec![];
        for elements in value {
            let mut rows = vec![];
            for element in elements {
                if !element.is_zero() {
                    rows.push(element.into())
                }
            }
            sparse_matrix.push(rows)
        }
        Self(sparse_matrix)
    }
}

#[derive(Debug)]
pub struct Element<F: PrimeField>(Wire, F);

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
