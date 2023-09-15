use crate::constraint::Constraint;
use crate::wire::Wire;

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

    pub(crate) fn wire(&mut self) -> Wire {
        let pointer = self.pointer;
        self.pointer += 1;
        Wire::new(pointer)
    }
}
