use crate::assignment::Assignment;
use crate::constraint::Constraint;

use zkstd::common::PrimeField;

pub(crate) struct Gadget<F: PrimeField> {
    pub(crate) r1cs: Vec<Constraint<F>>,
}

impl<F: PrimeField> Gadget<F> {
    pub(crate) fn new(r1cs: Vec<Constraint<F>>) -> Self {
        Self { r1cs }
    }

    pub(crate) fn is_sat(&self, assignments: Vec<Assignment<F>>) -> bool {
        true
    }
}
