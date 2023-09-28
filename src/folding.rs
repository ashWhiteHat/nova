use crate::commitment::CommitmentScheme;
use crate::committed_relaxed_r1cs::{
    CommittedRelaxedR1csInstance, Instance as CommittedRelaxedR1csInstanceData,
    Witness as CommittedRelaxedR1csWitness,
};
use crate::matrix::DenseVectors;
use crate::r1cs::{R1csStructure, Witness as R1csWitness};
use crate::relaxed_r1cs::commit_relaxed_r1cs_instance;
use crate::transcript::{ChallengeTranscript, Transcript};

use zkstd::common::{CurveAffine, PrimeField, Ring};

pub struct FoldingScheme<C: CurveAffine> {
    /// r1cs martix A, B and C
    pub r1cs: R1csStructure<C::Scalar>,
    /// witness for one instance
    pub z1: Vec<C::Scalar>,
    /// witness for other instance
    pub z2: Vec<C::Scalar>,
    /// commitment scheme
    pub cs: CommitmentScheme<C>,
    /// randomness
    pub r: C::Scalar,
}

impl<C: CurveAffine> FoldingScheme<C> {
    pub fn new(
        r1cs: R1csStructure<C::Scalar>,
        z1: Vec<C::Scalar>,
        z2: Vec<C::Scalar>,
        cs: CommitmentScheme<C>,
        r: C::Scalar,
    ) -> Self {
        Self {
            r1cs,
            z1,
            z2,
            cs,
            r,
        }
    }

    pub fn folding(&self) -> CommittedRelaxedR1csInstance<C> {
        // construct r1cs instance
        let r1cs_instance1 = self.r1cs.instantiate(&self.z1);
        let r1cs_instance2 = self.r1cs.instantiate(&self.z2);
        let r1cs_witness1 = r1cs_instance1.witness.clone();
        let r1cs_witness2 = r1cs_instance2.witness.clone();

        // construct relaxed r1cs instance
        let relaxed_r1cs_instance1 = r1cs_instance1.relax();
        let relaxed_r1cs_instance2 = r1cs_instance2.relax();

        // commit relaxed r1cs instance
        let (r_e1, r_w1) = (C::Scalar::one(), C::Scalar::one());
        let (r_e2, r_w2) = (C::Scalar::one(), C::Scalar::one());
        let committed_relaxed_r1cs_instance1 =
            commit_relaxed_r1cs_instance(relaxed_r1cs_instance1, r_e1, r_w1, &self.cs);
        let committed_relaxed_r1cs_instance2 =
            commit_relaxed_r1cs_instance(relaxed_r1cs_instance2, r_e2, r_w2, &self.cs);

        // output folded committed relaxed r1cs instance
        self.prove(
            r1cs_witness1,
            r1cs_witness2,
            committed_relaxed_r1cs_instance1,
            committed_relaxed_r1cs_instance2,
        )
    }

    fn prove(
        &self,
        w1: R1csWitness<C::Scalar>,
        w2: R1csWitness<C::Scalar>,
        committed1: CommittedRelaxedR1csInstance<C>,
        committed2: CommittedRelaxedR1csInstance<C>,
    ) -> CommittedRelaxedR1csInstance<C> {
        // 0. setup params
        let rt = C::Scalar::one();
        let u1 = committed1.instance.u;
        let u2 = committed2.instance.u;
        let mut tr = ChallengeTranscript::<C>::init(b"fold-relaxed-r1cs");
        committed1.instance.append_to_transcript(&mut tr);
        committed2.instance.append_to_transcript(&mut tr);

        // 1. compute cross term
        let t = self.compute_cross_term(w1, w2, u1, u2);
        let overline_t = self.cs.commit(&t, &rt);
        tr.append_point(b"commit-t", &overline_t);

        // 2. sample challenge
        let r = <Transcript as ChallengeTranscript<C>>::challenge_scalar(&mut tr, b"random-scalar");

        // 3. output folded instance
        let folded_committed_r1cs_instance = Self::fold_committed_r1cs_instance(
            committed1.instance,
            committed2.instance,
            r,
            overline_t,
        );

        // 4. output folded witness
        let folded_committed_r1cs_witness =
            Self::fold_committed_r1cs_witness(committed1.witness, committed2.witness, r, t, rt);

        CommittedRelaxedR1csInstance {
            committed_relaxed_r1cs: self.r1cs.relax().commit(),
            instance: folded_committed_r1cs_instance,
            witness: folded_committed_r1cs_witness,
        }
    }

    /// (A · Z2) ◦ (B · Z1) + (A · Z1) ◦ (B · Z2) - c1(C · Z2) - c2(C · Z1)
    fn compute_cross_term(
        &self,
        w1: R1csWitness<C::Scalar>,
        w2: R1csWitness<C::Scalar>,
        c1: C::Scalar,
        c2: C::Scalar,
    ) -> DenseVectors<C::Scalar> {
        let R1csStructure { m, l: _, a, b, c } = self.r1cs.clone();
        let (x1, w1) = w1.get();
        let (x2, w2) = w2.get();

        // r1cs and z vectors dot product
        let az2 = a.prod(m, &x2, &w2);
        let bz1 = b.prod(m, &x1, &w1);
        let az1 = a.prod(m, &x1, &w1);
        let bz2 = b.prod(m, &x2, &w2);
        let cz2 = c.prod(m, &x2, &w2);
        let cz1 = c.prod(m, &x1, &w1);

        // dense vectors multiplication a.k.a Hadamard product
        let az2bz1 = az2 * bz1;
        let az1bz2 = az1 * bz2;

        // dense vectors and random scalar multiplication
        let c1cz2 = cz2 * c1;
        let c2cz1 = cz1 * c2;

        // final addition and subtraction
        az2bz1 + az1bz2 - c1cz2 - c2cz1
    }

    fn fold_committed_r1cs_instance(
        instance1: CommittedRelaxedR1csInstanceData<C>,
        instance2: CommittedRelaxedR1csInstanceData<C>,
        r: C::Scalar,
        overline_t: C,
    ) -> CommittedRelaxedR1csInstanceData<C> {
        let r2 = r.square();
        let (overline_e1, u1, overline_w1, x1) = instance1.get();
        let (overline_e2, u2, overline_w2, x2) = instance2.get();
        let overline_e = (overline_e1 + overline_t * r + overline_e2 * r2).into();
        let overline_w = (overline_w1 + overline_w2 * r).into();
        CommittedRelaxedR1csInstanceData {
            overline_e,
            u: u1 + r * u2,
            overline_w,
            x: x1 + x2 * r,
        }
    }

    fn fold_committed_r1cs_witness(
        witness1: CommittedRelaxedR1csWitness<C::Scalar>,
        witness2: CommittedRelaxedR1csWitness<C::Scalar>,
        r: C::Scalar,
        t: DenseVectors<C::Scalar>,
        rt: C::Scalar,
    ) -> CommittedRelaxedR1csWitness<C::Scalar> {
        let r2 = r.square();
        let (e1, r_e1, w1, r_w1) = witness1.get();
        let (e2, r_e2, w2, r_w2) = witness2.get();
        let e = e1 + t * r + e2 * r2;
        let r_e = r_e1 + r * rt + r2 * r_e2;
        let w = w1 + w2 * r;
        let r_w = r_w1 + r * r_w2;
        CommittedRelaxedR1csWitness { e, r_e, w, r_w }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{example_r1cs, example_r1cs_witness};

    use bls_12_381::{Fr as Scalar, G1Affine as Affine};
    use rand_core::OsRng;
    use zkstd::common::PrimeField;

    // mocked Fiat-Shamir transform
    // r ← H(x1, x2, T)
    fn challenge_r<F: PrimeField>() -> F {
        F::random(OsRng)
    }

    #[test]
    fn r1cs_folding_test() {
        let r1cs = example_r1cs();
        let z1 = example_r1cs_witness(3);
        let z2 = example_r1cs_witness(4);

        let r: Scalar = challenge_r();
        let n = r1cs.m.next_power_of_two() as u64;
        let cs: CommitmentScheme<Affine> = CommitmentScheme::new(n, OsRng);

        let folding_scheme = FoldingScheme::new(r1cs, z1, z2, cs, r);
        let folded_instance = folding_scheme.folding();
        assert!(folded_instance.is_sat())
    }
}
