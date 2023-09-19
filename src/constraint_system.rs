use crate::r1cs::{Element, R1cs};
use crate::wire::Wire;

use zkstd::common::PrimeField;

pub struct ConstraintSystem<F: PrimeField> {
    r1cs: R1cs<F>,
    instances: Vec<F>,
    witnessess: Vec<F>,
}

impl<F: PrimeField> ConstraintSystem<F> {
    /// init constraint system with first instance one
    pub fn new() -> Self {
        Self {
            r1cs: R1cs::default(),
            instances: vec![F::one()],
            witnessess: vec![],
        }
    }

    /// assign instance value to constraint system
    pub fn public_wire(&mut self, instance: F) -> Wire {
        let index = self.instances.len();
        self.instances.push(instance);
        Wire::instance(index)
    }

    /// assign witness value to constraint system
    pub fn private_wire(&mut self, witness: F) -> Wire {
        let index = self.witnessess.len();
        self.witnessess.push(witness);
        Wire::witness(index)
    }

    /// constrain a * b == c
    pub fn mul_constraint(&mut self, a: Wire, b: Wire, c: Wire) {
        self.enable_constraint(a, b, c)
    }

    /// constrain a == b
    pub fn equal_constraint(&mut self, a: Wire, b: Wire) {
        self.enable_constraint(a, F::one(), b)
    }

    /// add constraint internally
    fn enable_constraint(
        &mut self,
        a: impl Into<Element<F>>,
        b: impl Into<Element<F>>,
        c: impl Into<Element<F>>,
    ) {
        self.r1cs.append(a, b, c)
    }

    /// check whether constraints satisfy
    pub fn is_sat(&self) -> bool {
        let R1cs { m, a, b, c } = &self.r1cs;
        (0..*m).all(|i| {
            let a_prod = self.dot_product(&a.0[i]);
            let b_prod = self.dot_product(&b.0[i]);
            let c_prod = self.dot_product(&c.0[i]);
            a_prod * b_prod == c_prod
        })
    }

    // dot product for each gate
    fn dot_product(&self, elements: &Vec<Element<F>>) -> F {
        elements.iter().fold(F::zero(), |sum, element| {
            let (wire, value) = (element.0, element.1);
            let coeff = match wire {
                Wire::Instance(index) => self.instances[index],
                Wire::Witness(index) => self.witnessess[index],
            };
            sum + coeff * value
        })
    }
}
