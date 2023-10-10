use crate::committed_relaxed_r1cs::{
    CommittedRelaxedR1csInstance, CommittedRelaxedR1csStructure,
    Instance as CommittedRelaxedR1csInstanceData, Witness as CommittedRelaxedR1csWitness,
};
use crate::matrix::{DenseVectors, SparseMatrix};
use crate::proof::IvcProof;
use crate::public_param::PedersenCommitment;
use crate::r1cs::{R1csStructure, Witness as R1csWitness};

use zkstd::common::{CurveAffine, PrimeField, Ring};

pub(crate) struct ProvingKey<C: CurveAffine> {
    pub(crate) pp: PedersenCommitment<C>,
    pub(crate) f: R1csStructure<C::Scalar>,
    pub(crate) i: usize,
}

impl<C: CurveAffine> ProvingKey<C> {
    pub(crate) fn new(pp: PedersenCommitment<C>, f: R1csStructure<C::Scalar>) -> Self {
        Self { pp, f, i: 0 }
    }

    pub(crate) fn recurse(
        &self,
        z0: Vec<C::Scalar>,
        zi: Vec<C::Scalar>,
        πi: IvcProof<C>,
    ) -> IvcProof<C> {
        if self.i == 0 {}
        let IvcProof {
            upper_pair,
            lower_pair,
        } = πi;
        let (folded_committed_r1cs_instance, overline_t) = self.prove(upper_pair, lower_pair);
    }

    fn prove(
        &self,
        upper_pair: (CommittedRelaxedR1csInstance<C>, R1csWitness<C::Scalar>),
        lower_pair: (CommittedRelaxedR1csInstance<C>, R1csWitness<C::Scalar>),
    ) -> (CommittedRelaxedR1csInstance<C>, C) {
        // 0. setup params
        let r = C::Scalar::one();
        let rt = C::Scalar::one();
        let ((upper_u, upper_w), (lower_u, lower_w)) = (upper_pair, lower_pair);

        // 1. compute cross term
        let t = self.compute_cross_term(upper_w, lower_w, upper_u.instance.u, lower_u.instance.u);
        let overline_t = self.pp.commit(&t, &rt);

        // 2. output folded instance
        let folded_committed_r1cs_instance =
            Self::fold_committed_r1cs_instance(upper_u.instance, lower_u.instance, r, overline_t);

        // 3. output folded witness
        let folded_committed_r1cs_witness =
            Self::fold_committed_r1cs_witness(upper_u.witness, lower_u.witness, r, t, rt);

        let folded_committed_r1cs = CommittedRelaxedR1csInstance {
            committed_relaxed_r1cs: self.f.relax().commit(),
            instance: folded_committed_r1cs_instance,
            witness: folded_committed_r1cs_witness,
        };

        (folded_committed_r1cs, overline_t)
    }

    /// (A · Z2) ◦ (B · Z1) + (A · Z1) ◦ (B · Z2) - c1(C · Z2) - c2(C · Z1)
    fn compute_cross_term(
        &self,
        w1: R1csWitness<C::Scalar>,
        w2: R1csWitness<C::Scalar>,
        c1: C::Scalar,
        c2: C::Scalar,
    ) -> DenseVectors<C::Scalar> {
        let Self { pp, f, i: _ } = self.clone();
        let R1csStructure { m, l: _, a, b, c } = f;
        let (x1, w1) = w1.get();
        let (x2, w2) = w2.get();

        // r1cs and z vectors dot product
        let az2 = a.prod(*m, &x2, &w2);
        let bz1 = b.prod(*m, &x1, &w1);
        let az1 = a.prod(*m, &x1, &w1);
        let bz2 = b.prod(*m, &x2, &w2);
        let cz2 = c.prod(*m, &x2, &w2);
        let cz1 = c.prod(*m, &x1, &w1);

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
