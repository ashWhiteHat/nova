mod blueprint;
mod witness;

pub(crate) use blueprint::R1cs;
pub(crate) use witness::R1csWitness;

use crate::matrix::Element;
use crate::relaxed_r1cs::RelaxedR1csInstance;
use crate::wire::Wire;

use zkstd::common::PrimeField;

#[derive(Debug, Default)]
pub struct R1csInstance<F: PrimeField> {
    pub(crate) r1cs: R1cs<F>,
    pub(crate) z: R1csWitness<F>,
}

impl<F: PrimeField> R1csInstance<F> {
    pub(crate) fn new(r1cs: &R1cs<F>, z: &Vec<F>) -> Self {
        let z = r1cs.instance_and_witness(z.to_vec());
        let r1cs = r1cs.clone();
        Self { r1cs, z }
    }

    pub(crate) fn relax(&self) -> RelaxedR1csInstance<F> {
        let relaxed_r1cs = self.r1cs.relax();
        let (witness, instance) = self.z.relax(self.r1cs.m);
        RelaxedR1csInstance {
            relaxed_r1cs,
            instance,
            witness,
        }
    }

    ///  check (A · Z) ◦ (B · Z) = C · Z
    pub(crate) fn is_sat(&self) -> bool {
        let R1cs { m, l: _, a, b, c } = self.r1cs.clone();
        (0..m).all(|i| {
            let a_prod = self.dot_product(&a[i]);
            let b_prod = self.dot_product(&b[i]);
            let c_prod = self.dot_product(&c[i]);
            a_prod * b_prod == c_prod
        })
    }

    // dot product for each gate
    fn dot_product(&self, elements: &Vec<Element<F>>) -> F {
        elements.iter().fold(F::zero(), |sum, element| {
            let (wire, value) = (element.0, element.1);
            let coeff = match wire {
                Wire::Witness(index) => self.z.w[index],
                Wire::Instance(index) => self.z.x[index],
                Wire::One => self.z.one,
            };
            sum + coeff * value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{R1cs, R1csInstance};
    use crate::tests::{array_to_witnessess, dense_to_sparse};

    use bls_12_381::Fr as Scalar;

    #[test]
    fn r1cs_test() {
        // R1CS for: x^3 + x + 5 = y
        // https://www.vitalik.ca/general/2016/12/10/qap.html
        let m = 4;
        let l = 1;
        let a = dense_to_sparse::<Scalar>(
            vec![
                vec![0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 0],
                vec![5, 0, 0, 0, 0, 1],
            ],
            l,
        );
        let b = dense_to_sparse::<Scalar>(
            vec![
                vec![0, 1, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0, 0],
            ],
            l,
        );
        let c = dense_to_sparse::<Scalar>(
            vec![
                vec![0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 1],
                vec![0, 0, 1, 0, 0, 0],
            ],
            l,
        );
        let r1cs = R1cs { m, l, a, b, c };
        let z = array_to_witnessess::<Scalar>(vec![1, 3, 35, 9, 27, 30]);
        let r1cs_instance = R1csInstance::new(&r1cs, &z);
        assert!(r1cs_instance.is_sat())
    }
}
