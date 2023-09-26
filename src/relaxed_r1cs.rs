mod blueprint;
mod instance;
mod witness;

pub(crate) use blueprint::RelaxedR1CS;
pub(crate) use instance::RelaxedR1csInstanceData;
pub(crate) use witness::{commit_relaxed_z, RelaxedR1csWitness};

use crate::commitment::CommitmentScheme;
use crate::committed_relaxed_r1cs::{CommittedRelaxedR1cs, CommittedRelaxedR1csInstance};
use crate::matrix::{DenseVectors, Element};
use crate::wire::Wire;

use zkstd::common::{CurveAffine, PrimeField};

pub(crate) struct RelaxedR1csInstance<F: PrimeField> {
    pub(crate) relaxed_r1cs: RelaxedR1CS<F>,
    pub(crate) instance: RelaxedR1csInstanceData<F>,
    pub(crate) witness: RelaxedR1csWitness<F>,
}

impl<F: PrimeField> RelaxedR1csInstance<F> {
    ///  check (A · Z) ◦ (B · Z) = u · (C · Z) + E
    pub(crate) fn is_sat(&self) -> bool {
        let RelaxedR1CS { m, l, a, b, c } = self.relaxed_r1cs.clone();
        let RelaxedR1csInstanceData { e, u, x } = self.instance.clone();
        (0..m).all(|i| {
            let a_prod = self.dot_product(&a[i]);
            let b_prod = self.dot_product(&b[i]);
            let c_prod = self.dot_product(&c[i]) * u;
            a_prod * b_prod == c_prod + e[i]
        })
    }

    // dot product for each gate
    fn dot_product(&self, elements: &Vec<Element<F>>) -> F {
        elements.iter().fold(F::zero(), |sum, element| {
            let (wire, value) = (element.0, element.1);
            let coeff = match wire {
                Wire::Witness(index) => self.witness.w[index],
                Wire::Instance(index) => self.witness.x[index],
                Wire::One => self.witness.u,
            };
            sum + coeff * value
        })
    }
}

pub(crate) fn commit_relaxed_r1cs<C: CurveAffine>(
    relaxed_r1cs_instance: &RelaxedR1csInstanceData<C::Scalar>,
    w: &DenseVectors<C::Scalar>,
    cs: &CommitmentScheme<C>,
) -> CommittedRelaxedR1cs<C> {
    let RelaxedR1csInstanceData { e, u, x } = relaxed_r1cs_instance;
    CommittedRelaxedR1cs {
        overline_e: cs.commit(e, u),
        u: *u,
        overline_w: cs.commit(w, u),
        x: x.clone(),
    }
}

pub(crate) fn commit_relaxed_r1cs_instance<C: CurveAffine>(
    relaxed_r1cs_instance: RelaxedR1csInstance<C::Scalar>,
    r_e: C::Scalar,
    r_w: C::Scalar,
    cs: &CommitmentScheme<C>,
) -> CommittedRelaxedR1csInstance<C> {
    let RelaxedR1csInstance {
        relaxed_r1cs,
        instance,
        witness,
    } = relaxed_r1cs_instance;
    let e = instance.e.clone();
    let RelaxedR1csWitness { x, w, u: _ } = &witness;
    let committed_relaxed_r1cs = commit_relaxed_r1cs(&instance, w, cs);
    let committed_relaxed_z = commit_relaxed_z(&witness, e, r_e, r_w);
    CommittedRelaxedR1csInstance {
        committed_relaxed_r1cs,
        committed_relaxed_z,
    }
}
