use crate::constraint::Constraint;
use crate::expression::Expression;
use crate::wire::Wire;

use zkstd::common::PrimeField;

pub struct ConstraintSystem<F: PrimeField> {
    r1cs: Vec<Constraint<F>>,
    instances: Vec<F>,
    witnessess: Vec<F>,
}

impl<F: PrimeField> ConstraintSystem<F> {
    /// init constraint system with first instance one
    pub fn new() -> Self {
        Self {
            r1cs: vec![],
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
        self.add_constraint(a, b, c)
    }

    /// constrain a == b
    pub fn equal_constraint(&mut self, a: Wire, b: Wire) {
        self.add_constraint(a, F::one(), b)
    }

    /// add constraint internally
    fn add_constraint(
        &mut self,
        a: impl Into<Expression<F>>,
        b: impl Into<Expression<F>>,
        c: impl Into<Expression<F>>,
    ) {
        self.r1cs
            .push(Constraint::new(a.into(), b.into(), c.into()))
    }

    /// check whether constraints satisfy
    pub fn is_sat(&self) -> bool {
        self.r1cs.iter().all(
            |Constraint {
                 left,
                 right,
                 output,
             }| {
                let a = self.dot_product(left);
                let b = self.dot_product(right);
                let c = self.dot_product(output);
                a * b == c
            },
        )
    }

    fn dot_product(&self, expression: &Expression<F>) -> F {
        expression
            .coeffs
            .iter()
            .fold(F::zero(), |sum, (wire, coeff)| {
                let value = match *wire {
                    Wire::Instance(index) => self.instances[index],
                    Wire::Witness(index) => self.witnessess[index],
                };
                sum + *coeff * value
            })
    }
}
