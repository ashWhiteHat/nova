mod blueprint;
mod instance;
mod witness;

pub(crate) use blueprint::RelaxedR1CS;
pub(crate) use instance::{commit_relaxed_r1cs_instance_data, RelaxedR1csInstanceData};
pub(crate) use witness::{commit_relaxed_witness, RelaxedR1csWitness};

use crate::commitment::CommitmentScheme;
use crate::committed_relaxed_r1cs::CommittedRelaxedR1csInstance;
use crate::matrix::Element;
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
        let RelaxedR1CS { m, l: _, a, b, c } = self.relaxed_r1cs.clone();
        let RelaxedR1csInstanceData { e, u, x: _ } = self.instance.clone();
        (0..m).all(|i| {
            let a_prod = self.dot_product(&a[i]);
            let b_prod = self.dot_product(&b[i]);
            // scalar by u
            let c_prod = self.dot_product(&c[i]) * u;
            // E addition
            a_prod * b_prod == c_prod + e[i]
        })
    }

    // dot product for each gate
    fn dot_product(&self, elements: &Vec<Element<F>>) -> F {
        elements.iter().fold(F::zero(), |sum, element| {
            let (wire, value) = element.get();
            let coeff = match wire {
                Wire::Witness(index) => self.witness.w[index],
                Wire::Instance(index) => self.witness.x[index],
                Wire::One => self.witness.u,
            };
            sum + coeff * value
        })
    }
}

pub(crate) fn commit_relaxed_r1cs_instance<C: CurveAffine>(
    relaxed_r1cs_instance: RelaxedR1csInstance<C::Scalar>,
    r_e: C::Scalar,
    r_w: C::Scalar,
    cs: &CommitmentScheme<C>,
) -> CommittedRelaxedR1csInstance<C> {
    let RelaxedR1csInstance {
        relaxed_r1cs: _,
        instance,
        witness,
    } = relaxed_r1cs_instance;
    let e = instance.e.clone();
    let RelaxedR1csWitness { x: _, w, u: _ } = &witness;
    let committed_relaxed_r1cs = commit_relaxed_r1cs_instance_data(&instance, w, cs);
    let committed_relaxed_z = commit_relaxed_witness(&witness, e, r_e, r_w);
    CommittedRelaxedR1csInstance {
        committed_relaxed_r1cs,
        committed_relaxed_z,
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::example_relaxed_r1cs_instance;

    use bls_12_381::Fr as Scalar;

    #[test]
    fn relaxed_r1cs_instance_test() {
        for i in 0..100 {
            let relaxed_r1cs_instance = example_relaxed_r1cs_instance::<Scalar>(i);
            assert!(relaxed_r1cs_instance.is_sat())
        }
    }
}
