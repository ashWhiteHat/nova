mod blueprint;
mod instance;
mod witness;

pub(crate) use blueprint::RelaxedR1csStructure;
pub(crate) use instance::Instance;
pub(crate) use witness::Witness;

use zkstd::common::PrimeField;

pub(crate) struct RelaxedR1csInstance<F: PrimeField> {
    pub(crate) relaxed_r1cs: RelaxedR1csStructure<F>,
    pub(crate) instance: Instance<F>,
    pub(crate) witness: Witness<F>,
}

#[cfg(test)]
use crate::matrix::Element;
#[cfg(test)]
use crate::wire::Wire;
#[cfg(test)]
impl<F: PrimeField> RelaxedR1csInstance<F> {
    ///  check (A · Z) ◦ (B · Z) = u · (C · Z) + E
    pub(crate) fn is_sat(&self) -> bool {
        let RelaxedR1csStructure { m, l: _, a, b, c } = self.relaxed_r1cs.clone();
        let Instance { e, u, x: _ } = self.instance.clone();
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

#[cfg(test)]
mod tests {
    use crate::tests::example_relaxed_r1cs_instance;

    use jub_jub::Fr as Scalar;

    #[test]
    fn relaxed_r1cs_instance_test() {
        for i in 0..100 {
            let relaxed_r1cs_instance = example_relaxed_r1cs_instance::<Scalar>(i);
            assert!(relaxed_r1cs_instance.is_sat())
        }
    }
}
