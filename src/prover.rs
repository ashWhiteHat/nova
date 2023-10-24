use crate::matrix::DenseVectors;
use crate::proof::IvcProof;
use crate::public_param::PedersenCommitment;
use crate::r1cs::{R1csInstance as R1cs, R1csStructure};
use crate::relaxed_r1cs::RelaxedR1csInstance as RelaxedR1cs;

use zkstd::common::{Ring, TwistedEdwardsAffine};

pub(crate) struct Prover<C: TwistedEdwardsAffine> {
    pub(crate) pp: PedersenCommitment<C>,
    pub(crate) f: R1csStructure<C>,
    pub(crate) i: usize,
}

impl<C: TwistedEdwardsAffine> Prover<C> {
    pub(crate) fn new(pp: PedersenCommitment<C>, f: R1csStructure<C>) -> Self {
        Self { pp, f, i: 0 }
    }

    pub(crate) fn recurse(
        &self,
        z0: Vec<C::Scalar>,
        zi: Vec<C::Scalar>,
        πi: IvcProof<C>,
    ) -> IvcProof<C> {
        if self.i == 0 {}
        πi
    }

    pub(crate) fn prove(&self, r1cs: R1cs<C>, relaxed_r1cs: RelaxedR1cs<C>) -> RelaxedR1cs<C> {
        let lc_random = C::Scalar::one();
        let (w0, x0) = (r1cs.witness.w.clone(), r1cs.instance.x.clone());
        let (w1, x1) = (
            relaxed_r1cs.witness.w.clone(),
            relaxed_r1cs.instance.x.clone(),
        );
        let u2 = relaxed_r1cs.instance.u;
        let t = self.compute_cross_term(w0, x0, w1, x1, u2);
        let commit_t = self.pp.commit(&t, &lc_random);
        let folded_instance = relaxed_r1cs
            .instance
            .fold(r1cs.instance, lc_random, commit_t);
        let folded_witness = relaxed_r1cs.witness.fold(r1cs.witness, lc_random, t);

        RelaxedR1cs {
            r1cs: self.f.clone(),
            instance: folded_instance,
            witness: folded_witness,
        }
    }

    pub(crate) fn compute_cross_term(
        &self,
        w0: DenseVectors<C::Scalar>,
        x0: DenseVectors<C::Scalar>,
        w1: DenseVectors<C::Scalar>,
        x1: DenseVectors<C::Scalar>,
        u2: C::Scalar,
    ) -> DenseVectors<C::Scalar> {
        let u1 = C::Scalar::one();
        let R1csStructure { m, l: _, a, b, c } = self.f.clone();

        // r1cs and z vectors dot product
        let az2 = a.prod(m, &x1, &w1);
        let bz1 = b.prod(m, &x0, &w0);
        let az1 = a.prod(m, &x0, &w0);
        let bz2 = b.prod(m, &x1, &w1);
        let cz2 = c.prod(m, &x1, &w1);
        let cz1 = c.prod(m, &x0, &w0);

        // dense vectors multiplication a.k.a Hadamard product
        let az2bz1 = az2 * bz1;
        let az1bz2 = az1 * bz2;

        // dense vectors and random scalar multiplication
        let c1cz2 = cz2 * u1;
        let c2cz1 = cz1 * u2;

        // final addition and subtraction
        az2bz1 + az1bz2 - c1cz2 - c2cz1
    }
}

#[cfg(test)]
mod tests {
    use super::Prover;
    use crate::public_param::PedersenCommitment;
    use crate::tests::{example_r1cs, example_r1cs_instance, example_relaxed_r1cs_instance};

    use jub_jub::JubjubAffine as Curve;
    use rand_core::OsRng;

    #[test]
    fn folding_test() {
        let r1cs = example_r1cs::<Curve>();
        let n = r1cs.m.next_power_of_two() as u64;
        let pp = PedersenCommitment::<Curve>::new(n, OsRng);
        let r1cs_instance = example_r1cs_instance::<Curve>(3);
        let relaxed_r1cs_instance = example_relaxed_r1cs_instance::<Curve>(4);
        let prover = Prover::new(pp, r1cs);
        let folded_r1cs_instance = prover.prove(r1cs_instance, relaxed_r1cs_instance);

        assert!(folded_r1cs_instance.is_sat())
    }
}
