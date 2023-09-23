use crate::commitment::CommitmentScheme;
use crate::matrix::DenseVectors;
use crate::r1cs::R1cs;
use crate::relaxed_r1cs::CommittedRelaxedR1CS;

use zkstd::common::CurveAffine;

pub struct FoldingScheme<C: CurveAffine> {
    pub r1cs: R1cs<C::Scalar>,
    pub x1: Vec<C::Scalar>,
    pub x2: Vec<C::Scalar>,
    pub w1: Vec<C::Scalar>,
    pub w2: Vec<C::Scalar>,
    pub cs: CommitmentScheme<C>,
    pub r: C::Scalar,
}

impl<C: CurveAffine> FoldingScheme<C> {
    pub fn new(
        r1cs: R1cs<C::Scalar>,
        x1: Vec<C::Scalar>,
        x2: Vec<C::Scalar>,
        w1: Vec<C::Scalar>,
        w2: Vec<C::Scalar>,
        cs: CommitmentScheme<C>,
        r: C::Scalar,
    ) -> Self {
        Self {
            r1cs,
            x1,
            x2,
            w1,
            w2,
            cs,
            r,
        }
    }

    pub fn folding(&self) {
        // convert r1cs instance to relaxed r1cs instance
        let relaxed_r1cs = self.r1cs.relax();
        // commit relaxed r1cs instance
        let crr1 = self
            .cs
            .commit_relaxed_r1cs(&relaxed_r1cs, &self.w1, &self.x1, &self.cs);
        let crr2 = self
            .cs
            .commit_relaxed_r1cs(&relaxed_r1cs, &self.w2, &self.x2, &self.cs);
        self.prove(crr1, crr2)
    }

    fn prove(&self, crr1: CommittedRelaxedR1CS<C>, crr2: CommittedRelaxedR1CS<C>) {
        // compute cross term
        let t = self.compute_cross_term(crr1.u, crr2.u);
    }

    /// (A · Z2) ◦ (B · Z1) + (A · Z1) ◦ (B · Z2) - c1(C · Z2) - c2(C · Z1)
    fn compute_cross_term(&self, c1: C::Scalar, c2: C::Scalar) -> DenseVectors<C::Scalar> {
        let R1cs { m, l: _, a, b, c } = self.r1cs.clone();

        // r1cs and z vectors dot product
        let az2 = a.prod(m, &self.x2, &self.w2);
        let bz1 = b.prod(m, &self.x1, &self.w1);
        let az1 = a.prod(m, &self.x1, &self.w1);
        let bz2 = b.prod(m, &self.x2, &self.w2);
        let cz2 = c.prod(m, &self.x2, &self.w2);
        let cz1 = c.prod(m, &self.x1, &self.w1);

        // dense vectors multiplication a.k.a Hadamard product
        let az2bz1 = az2 * bz1;
        let az1bz2 = az1 * bz2;

        // dense vectors and random scalar multiplication
        let c1cz2 = cz2 * c1;
        let c2cz1 = cz1 * c2;

        // final addition and subtraction
        az2bz1 + az1bz2 - c1cz2 - c2cz1
    }
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

        let folding_scheme = FoldingScheme::new(r1cs, x1, x2, w1, w2, cs, r);
        folding_scheme.folding();
    }
}
