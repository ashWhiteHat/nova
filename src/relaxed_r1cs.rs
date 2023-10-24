mod blueprint;
mod instance;
mod witness;

pub(crate) use instance::Instance;
pub(crate) use witness::Witness;

use crate::r1cs::R1csStructure;
use zkstd::common::TwistedEdwardsAffine;

pub(crate) struct RelaxedR1csInstance<C: TwistedEdwardsAffine> {
    pub(crate) r1cs: R1csStructure<C>,
    pub(crate) instance: Instance<C>,
    pub(crate) witness: Witness<C>,
}

#[cfg(test)]
use crate::matrix::Element;
#[cfg(test)]
use crate::wire::Wire;
#[cfg(test)]
use zkstd::common::Group;
#[cfg(test)]
impl<C: TwistedEdwardsAffine> RelaxedR1csInstance<C> {
    ///  check (A · Z) ◦ (B · Z) = u · (C · Z) + E
    pub(crate) fn is_sat(&self) -> bool {
        let R1csStructure { m, l: _, a, b, c } = self.r1cs.clone();
        let e = self.witness.e.clone();
        let u = self.instance.u.clone();
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
    fn dot_product(&self, elements: &Vec<Element<C::Scalar>>) -> C::Scalar {
        elements.iter().fold(C::Scalar::zero(), |sum, element| {
            let (wire, value) = element.get();
            let coeff = match wire {
                Wire::Witness(index) => self.witness.w[index],
                Wire::Instance(index) => self.instance.x[index],
                Wire::One => self.instance.u,
            };
            sum + coeff * value
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::example_relaxed_r1cs_instance;

    use jub_jub::JubjubAffine as Curve;

    #[test]
    fn relaxed_r1cs_instance_test() {
        for i in 0..100 {
            let relaxed_r1cs_instance = example_relaxed_r1cs_instance::<Curve>(i);
            assert!(relaxed_r1cs_instance.is_sat())
        }
    }
}
