use crate::constraint::Constraint;
use crate::wire::Wire;

use zkstd::common::PrimeField;

pub struct ConstraintSystem<F: PrimeField> {
    constraints: Vec<Constraint<F>>,
    instances: Vec<F>,
    witnessess: Vec<F>,
}

impl<F: PrimeField> ConstraintSystem<F> {
    pub fn new() -> Self {
        Self {
            constraints: vec![],
            instances: vec![F::one()],
            witnessess: vec![],
        }
    }

    /// assign instance value to constraint system
    pub fn public_wire(&mut self, instance: F) -> Wire {
        let index = self.instances.len();
        self.instances.push(instance);
        Wire::public(index)
    }

    /// assign witness value to constraint system
    pub fn private_wire(&mut self, witness: F) -> Wire {
        let index = self.witnessess.len();
        self.witnessess.push(witness);
        Wire::private(index)
    }
}
