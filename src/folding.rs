use crate::commitment::CommitmentScheme;
use crate::r1cs::R1cs;

use zkstd::common::CurveAffine;

pub(crate) fn folding<C: CurveAffine>(
    r1cs: R1cs<C::Scalar>,
    x1: Vec<C::Scalar>,
    x2: Vec<C::Scalar>,
    w1: Vec<C::Scalar>,
    w2: Vec<C::Scalar>,
    cs: &CommitmentScheme<C>,
) {
    let relaxed_r1cs = r1cs.relax();
    let crr1 = cs.commit_relaxed_r1cs(&relaxed_r1cs, w1, x1, cs);
    let crr2 = cs.commit_relaxed_r1cs(&relaxed_r1cs, w2, x2, cs);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{example_r1cs_instance, example_r1cs_witness};

    use bls_12_381::{Fr as Scalar, G1Affine as Affine};
    use rand_core::OsRng;
    use zkstd::common::PrimeField;

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
        let (x1, w1) = r1cs.instance_and_witness(z1);
        let (x2, w2) = r1cs.instance_and_witness(z2);
        let r: Scalar = challenge_r();
        let n = r1cs.m.next_power_of_two() as u64;
        let cs: CommitmentScheme<Affine> = CommitmentScheme::new(n, OsRng);

        folding(r1cs, x1, x2, w1, w2, &cs);
    }
}
