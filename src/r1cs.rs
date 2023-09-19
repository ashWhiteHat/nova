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

    pub(crate) fn increment(&mut self) {
        self.m += 1
    }
}

fn dot_product<F: PrimeField>(elements: &Vec<Element<F>>, witnesses: &Vec<F>) -> F {
    elements.iter().fold(F::zero(), |sum, element| {
        let (wire, value) = (element.0, element.1);
        let index = match wire {
            Wire::Instance(index) => index,
            Wire::Witness(index) => index,
        };
        sum + witnesses[index] * value
    })
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
    use super::{Element, PrimeField, R1cs, SparseMatrix, Wire};

    use bls_12_381::Fr as Scalar;

    fn array_to_witnessess<F: PrimeField>(witnesses: Vec<u64>) -> Vec<F> {
        witnesses
            .iter()
            .map(|witness| F::from(*witness))
            .collect::<Vec<_>>()
    }

    fn dense_to_sparse<F: PrimeField>(value: Vec<Vec<u64>>) -> SparseMatrix<F> {
        let sparse_matrix = value
            .iter()
            .map(|elements| {
                elements
                    .iter()
                    .enumerate()
                    .map(|(index, element)| Element(Wire::instance(index), F::from(*element)))
                    .filter(|element| element.1 != F::zero())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        SparseMatrix(sparse_matrix)
    }

    fn dot_product<F: PrimeField>(elements: &Vec<Element<F>>, witnesses: &Vec<F>) -> F {
        elements.iter().fold(F::zero(), |sum, element| {
            let (wire, value) = (element.0, element.1);
            let index = match wire {
                Wire::Instance(index) => index,
                Wire::Witness(index) => index,
            };
            sum + witnesses[index] * value
        })
    }

    fn is_satisfy<F: PrimeField>(r1cs: R1cs<F>, witnesses: Vec<F>) -> bool {
        let R1cs { m, a, b, c } = r1cs;
        (0..m).all(|i| {
            let a_prod = dot_product(&a.0[i], &witnesses);
            let b_prod = dot_product(&b.0[i], &witnesses);
            let c_prod = dot_product(&c.0[i], &witnesses);
            a_prod * b_prod == c_prod
        })
    }

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
        assert!(is_satisfy(r1cs, z))
    }
}
