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
    pub(crate) fn product(&self, witnesses: &Vec<F>) -> F {
        self.0.iter().fold(F::zero(), |mut sum, elements| {
            for element in elements {
                let (wire, value) = (element.0, element.1);
                let index = match wire {
                    Wire::Instance(index) => index,
                    Wire::Witness(index) => index,
                };
                sum += witnesses[index] * value;
            }
            sum
        })
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

#[cfg(test)]
mod tests {
    use super::{Element, PrimeField, R1cs, SparseMatrix, Wire};

    use bls_12_381::Fr as Scalar;

    fn index_value_to_element<F: PrimeField>(wire: Wire, value: F) -> Element<F> {
        Element(wire, value)
    }

    fn dense_to_sparse<F: PrimeField>(value: Vec<Vec<u64>>) -> SparseMatrix<F> {
        let mut sparse_matrix = vec![];
        for elements in value {
            let mut rows = vec![];
            for (index, element) in elements.iter().enumerate() {
                if !(*element == 0) {
                    let instance = Wire::instance(index);
                    let value = F::from(*element).into();
                    rows.push(index_value_to_element(instance, value))
                }
            }
            sparse_matrix.push(rows)
        }
        SparseMatrix(sparse_matrix)
    }

    fn is_satisfy<F: PrimeField>(r1cs: R1cs<F>, witnesses: Vec<u64>) -> bool {
        let witnesses = witnesses.iter().map(|witness| F::from(*witness)).collect();
        let a = r1cs.a.product(&witnesses);
        let b = r1cs.b.product(&witnesses);
        let c = r1cs.c.product(&witnesses);
        println!("a: {:?} b: {:?} c: {:?}", a, b, c);
        a * b == c
    }

    #[test]
    fn r1cs_test() {
        // R1CS for: x^3 + x + 5 = y
        // https://www.vitalik.ca/general/2016/12/10/qap.html
        let m = 3;
        let a = dense_to_sparse::<Scalar>(vec![
            vec![0, 1, 0, 0, 0, 0],
            // vec![0, 0, 0, 1, 0, 0],
            // vec![0, 1, 0, 0, 1, 0],
            // vec![5, 0, 0, 0, 0, 1],
        ]);
        let b = dense_to_sparse::<Scalar>(vec![
            vec![0, 1, 0, 0, 0, 0],
            // vec![0, 1, 0, 0, 0, 0],
            // vec![1, 0, 0, 0, 0, 0],
            // vec![1, 0, 0, 0, 0, 0],
        ]);
        let c = dense_to_sparse::<Scalar>(vec![
            vec![0, 0, 0, 1, 0, 0],
            // vec![0, 0, 0, 0, 1, 0],
            // vec![0, 0, 0, 0, 0, 1],
            // vec![0, 0, 1, 0, 0, 0],
        ]);
        let r1cs = R1cs { m, a, b, c };
        let z = vec![1, 3, 35, 9, 27, 30];
        assert!(is_satisfy(r1cs, z))
    }
}
