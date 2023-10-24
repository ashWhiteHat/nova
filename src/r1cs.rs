mod blueprint;
mod instance;
mod witness;

pub(crate) use blueprint::R1csStructure;
pub(crate) use instance::Instance;
pub(crate) use witness::Witness;

use crate::matrix::Element;
use crate::relaxed_r1cs::RelaxedR1csInstance;
use crate::wire::Wire;

use zkstd::common::{Group, TwistedEdwardsAffine};

#[derive(Debug, Default)]
pub struct R1csInstance<C: TwistedEdwardsAffine> {
    pub(crate) r1cs: R1csStructure<C>,
    pub(crate) instance: Instance<C::Scalar>,
    pub(crate) witness: Witness<C>,
}

impl<C: TwistedEdwardsAffine> R1csInstance<C> {
    pub(crate) fn new(r1cs: &R1csStructure<C>, witness: &Vec<C::Scalar>) -> Self {
        let (instance, witness) = r1cs.instance_and_witness(witness);
        let r1cs = r1cs.clone();
        Self {
            r1cs,
            instance,
            witness,
        }
    }

    pub(crate) fn relax(&self) -> RelaxedR1csInstance<C> {
        let r1cs = self.r1cs.clone();
        let (witness, instance) = self.witness.relax(self.r1cs.m);
        RelaxedR1csInstance {
            r1cs,
            instance,
            witness,
        }
    }

    ///  check (A · Z) ◦ (B · Z) = C · Z
    pub(crate) fn is_sat(&self) -> bool {
        let R1csStructure { m, l: _, a, b, c } = self.r1cs.clone();
        (0..m).all(|i| {
            let a_prod = self.dot_product(&a[i]);
            let b_prod = self.dot_product(&b[i]);
            let c_prod = self.dot_product(&c[i]);
            a_prod * b_prod == c_prod
        })
    }

    // dot product for each gate
    fn dot_product(&self, elements: &Vec<Element<C::Scalar>>) -> C::Scalar {
        elements.iter().fold(C::Scalar::zero(), |sum, element| {
            let (wire, value) = (element.0, element.1);
            let coeff = match wire {
                Wire::Witness(index) => self.witness.w[index],
                Wire::Instance(index) => self.witness.x[index],
                Wire::One => self.witness.one,
            };
            sum + coeff * value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{R1csInstance, R1csStructure};
    use crate::tests::{example_r1cs, example_r1cs_witness};

    use jub_jub::JubjubAffine as Curve;

    #[test]
    fn r1cs_instance_test() {
        let r1cs: R1csStructure<Curve> = example_r1cs();
        for i in 0..100 {
            let z = example_r1cs_witness(i);
            let r1cs_instance = R1csInstance::new(&r1cs, &z);
            assert!(r1cs_instance.is_sat())
        }
    }
}
