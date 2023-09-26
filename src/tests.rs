use crate::matrix::{Element, SparseMatrix};
use crate::r1cs::R1cs;
use crate::wire::Wire;

use zkstd::common::PrimeField;

pub(crate) fn array_to_witnessess<F: PrimeField>(witnesses: Vec<u64>) -> Vec<F> {
    witnesses
        .iter()
        .skip(1)
        .map(|witness| F::from(*witness))
        .collect::<Vec<_>>()
}

pub(crate) fn dense_to_sparse<F: PrimeField>(value: Vec<Vec<u64>>, l: usize) -> SparseMatrix<F> {
    let sparse_matrix = value
        .iter()
        .map(|elements| {
            elements
                .iter()
                .enumerate()
                .map(|(index, element)| {
                    if index == 0 {
                        Element(Wire::One, F::from(*element))
                    } else if index <= l {
                        let index = index - 1;
                        Element(Wire::instance(index), F::from(*element))
                    } else {
                        let index = index - 1 - l;
                        Element(Wire::witness(index), F::from(*element))
                    }
                })
                .filter(|element| element.1 != F::zero())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    SparseMatrix(sparse_matrix)
}

/// R1CS for: x^3 + x + 5 = y
/// https://www.vitalik.ca/general/2016/12/10/qap.html
pub(crate) fn example_r1cs_instance<F: PrimeField>() -> R1cs<F> {
    let m = 4;
    let l = 1;
    let a = dense_to_sparse(
        vec![
            vec![0, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 0],
            vec![5, 0, 0, 0, 0, 1],
        ],
        l,
    );
    let b = dense_to_sparse(
        vec![
            vec![0, 1, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0],
        ],
        l,
    );
    let c = dense_to_sparse(
        vec![
            vec![0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 0, 1],
            vec![0, 0, 1, 0, 0, 0],
        ],
        l,
    );
    R1cs { m, l, a, b, c }
}

pub(crate) fn example_r1cs_witness<F: PrimeField>(input: u64) -> Vec<F> {
    array_to_witnessess(vec![
        1,
        input,                             // x
        input * input * input + input + 5, // x^3 + x + 5
        input * input,                     // x^2
        input * input * input,             // x^2 * x
        input * input * input + input,     // x^3 + x
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::r1cs::R1csInstance;

    use bls_12_381::Fr as Scalar;

    #[test]
    fn example_r1cs_test() {
        let r1cs: R1cs<Scalar> = example_r1cs_instance();
        for i in 0..100 {
            let z = example_r1cs_witness(i);
            let r1cs_instance = R1csInstance::new(&r1cs, &z);
            assert!(r1cs_instance.is_sat())
        }
    }
}
