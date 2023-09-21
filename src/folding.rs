use crate::r1cs::R1cs;

use zkstd::common::PrimeField;

pub(crate) fn folding<F: PrimeField>(
    r1cs: R1cs<F>,
    x1: Vec<F>,
    x2: Vec<F>,
    w1: Vec<F>,
    w2: Vec<F>,
) {
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{example_r1cs_instance, example_r1cs_witness};

    use bls_12_381::Fr as Scalar;
    use rand_core::OsRng;

    // return instances and witnesses
    fn split_witness<F: PrimeField>(witnesses: Vec<F>) -> (Vec<F>, Vec<F>) {
        // public inputs size
        let l = 1;
        // first one const witness offset
        let offset = l + 1;
        (witnesses[1..offset].to_vec(), witnesses[offset..].to_vec())
    }

    // mocked Fiat-Shamir transform
    // r ← H(x1, x2, T)
    fn challenge_r<F: PrimeField>() -> F {
        F::random(OsRng)
    }

    #[test]
    fn folding_test() {
        let r1cs: R1cs<Scalar> = example_r1cs_instance();
        let z1 = example_r1cs_witness(3);
        let z2 = example_r1cs_witness(4);
        let (x1, w1) = split_witness(z1);
        let (x2, w2) = split_witness(z2);
        let r = challenge_r();

        folding(r1cs, x1, x2, w1, w2);
    }
}
