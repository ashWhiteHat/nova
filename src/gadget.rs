use crate::constraint::Constraint;

use zkstd::common::PrimeField;

pub(crate) struct Builder<F: PrimeField> {
    pointer: usize,
    r1cs: Vec<Constraint<F>>,
}

impl<F: PrimeField> Builder<F> {
    pub(crate) fn new() -> Self {
        Builder {
            pointer: 1,
            r1cs: Vec::new(),
        }
    }
}
