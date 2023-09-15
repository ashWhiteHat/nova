use crate::constraint::Constraint;

use zkstd::common::PrimeField;

pub(crate) struct Gadget<F: PrimeField> {
    pub(crate) r1cs: Vec<Constraint<F>>,
}

impl<F: PrimeField> Gadget<F> {
    pub(crate) fn new(r1cs: Vec<Constraint<F>>) -> Self {
        Self { r1cs }
    }
}
