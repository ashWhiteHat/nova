use crate::commitment::CommitmentScheme;
use crate::matrix::DenseVectors;
use crate::r1cs::R1cs;
use crate::relaxed_r1cs::{CommittedRelaxedR1CS, RelaxedR1CSInstance};

use zkstd::common::{CurveAffine, PrimeField, Ring};

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
        // construct relaxed r1cs instance
        let relaxed_r1cs_instance1 = relaxed_r1cs.to_instance(&self.x1, &self.w1);
        let relaxed_r1cs_instance2 = relaxed_r1cs.to_instance(&self.x2, &self.w2);
        // commit relaxed r1cs instance
        let committed_relaxed_r1cs_instance1 = self
            .cs
            .commit_relaxed_r1cs_instance(&relaxed_r1cs_instance1);
        let committed_relaxed_r1cs_instance2 = self
            .cs
            .commit_relaxed_r1cs_instance(&relaxed_r1cs_instance2);
        self.prove(
            (relaxed_r1cs_instance1, relaxed_r1cs_instance2),
            (
                committed_relaxed_r1cs_instance1,
                committed_relaxed_r1cs_instance2,
            ),
        )
    }

    fn prove(
        &self,
        instance_pair: (
            RelaxedR1CSInstance<C::Scalar>,
            RelaxedR1CSInstance<C::Scalar>,
        ),
        committed_pair: (CommittedRelaxedR1CS<C>, CommittedRelaxedR1CS<C>),
    ) {
        // 0. setup params
        let rt = C::Scalar::one();
        let r2 = self.r.square();
        let (instance1, instance2) = instance_pair;
        let (committed1, committed2) = committed_pair;

        // 1. compute cross term
        let t = self.compute_cross_term(committed1.u, committed2.u);
        let overline_t = self.cs.commit(&t.0, rt);

        // 2. sample challenge
        // TODO: should be replaced by transcript
        let r = self.r;

        // 3. output folded instance
        let overline_e =
            committed1.overline_e.to_extended() + overline_t * r + committed1.overline_e * r2;
        let u = committed1.u + r * committed2.u;
        let overline_w = committed1.overline_w.to_extended() + committed2.overline_w * r;
        let x = DenseVectors(committed1.x) + DenseVectors(committed2.x) * r;

        // 4. output folded witness
        let e = DenseVectors(instance1.e) + t * r + DenseVectors(instance2.e) * r2;
        let r_e = instance1.u + r * rt + instance1.u * r2;
        let w = DenseVectors(instance1.w) + DenseVectors(instance2.w) * r;
        let r_w = instance1.u + r * instance2.u;
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
