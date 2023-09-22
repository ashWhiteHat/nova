use crate::matrix::{Element, SparseMatrix};
use crate::r1cs::R1cs;
use crate::wire::Wire;

use zkstd::common::PrimeField;

pub(crate) fn array_to_witnessess<F: PrimeField>(witnesses: Vec<u64>) -> Vec<F> {
    witnesses
        .iter()
        .map(|witness| F::from(*witness))
        .collect::<Vec<_>>()
}

pub(crate) fn dense_to_sparse<F: PrimeField>(value: Vec<Vec<u64>>) -> SparseMatrix<F> {
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

pub(crate) fn dot_product<F: PrimeField>(elements: &Vec<Element<F>>, witnesses: &Vec<F>) -> F {
    elements.iter().fold(F::zero(), |sum, element| {
        let (wire, value) = (element.0, element.1);
        let index = match wire {
            Wire::Instance(index) => index,
            Wire::Witness(index) => index,
        };
        sum + witnesses[index] * value
    })
}

pub(crate) fn is_satisfy<F: PrimeField>(r1cs: &R1cs<F>, witnesses: Vec<F>) -> bool {
    let R1cs { m, l: _, a, b, c } = r1cs;
    (0..*m).all(|i| {
        let a_prod = dot_product(&a.0[i], &witnesses);
        let b_prod = dot_product(&b.0[i], &witnesses);
        let c_prod = dot_product(&c.0[i], &witnesses);
        a_prod * b_prod == c_prod
    })
}

pub(crate) fn example_r1cs_instance<F: PrimeField>() -> R1cs<F> {
    let m = 4;
    let l = 1;
    let a = dense_to_sparse(vec![
        vec![0, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0],
        vec![0, 1, 0, 0, 1, 0],
        vec![5, 0, 0, 0, 0, 1],
    ]);
    let b = dense_to_sparse(vec![
        vec![0, 1, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 0],
    ]);
    let c = dense_to_sparse(vec![
        vec![0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 1, 0],
        vec![0, 0, 0, 0, 0, 1],
        vec![0, 0, 1, 0, 0, 0],
    ]);
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

    use bls_12_381::Fr as Scalar;

    #[test]
    fn example_r1cs_test() {
        let r1cs: R1cs<Scalar> = example_r1cs_instance();
        for i in 0..100 {
            let witnesses = example_r1cs_witness(i);
            assert!(is_satisfy(&r1cs, witnesses))
        }
    }
}
