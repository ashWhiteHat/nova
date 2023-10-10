use crate::committed_relaxed_r1cs::CommittedRelaxedR1csInstance;
use crate::matrix::{Element, SparseMatrix};
use crate::public_param::PedersenCommitment;
use crate::r1cs::{R1csInstance, R1csStructure};
use crate::relaxed_r1cs::{commit_relaxed_r1cs_instance, RelaxedR1csInstance};
use crate::wire::Wire;

use rand_core::OsRng;
use zkstd::common::{CurveAffine, PrimeField, Ring};

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
pub(crate) fn example_r1cs<F: PrimeField>() -> R1csStructure<F> {
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
    R1csStructure { m, l, a, b, c }
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

pub(crate) fn example_relaxed_r1cs_instance<F: PrimeField>(input: u64) -> RelaxedR1csInstance<F> {
    let r1cs = example_r1cs();
    let z = example_r1cs_witness(input);
    let r1cs_instance = R1csInstance::new(&r1cs, &z);
    r1cs_instance.relax()
}

pub(crate) fn example_committed_relaxed_r1cs_instance<C: CurveAffine>(
    input: u64,
) -> CommittedRelaxedR1csInstance<C> {
    let r1cs = example_r1cs();
    let z = example_r1cs_witness(input);
    let r1cs_instance = R1csInstance::new(&r1cs, &z);
    let relaxed_r1cs_instance = r1cs_instance.relax();
    let (r_e, r_w) = (C::Scalar::one(), C::Scalar::one());
    let n = r1cs.m.next_power_of_two() as u64;
    let cs = PedersenCommitment::new(n, OsRng);
    commit_relaxed_r1cs_instance(relaxed_r1cs_instance, r_e, r_w, &cs)
}
