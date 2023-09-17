use crate::wire::Wire;

use zkstd::common::PrimeField;

#[derive(Debug)]
pub struct Assignment<F: PrimeField>(pub(crate) (Wire, F));

impl<F: PrimeField> Assignment<F> {
    pub fn new(wire: Wire, value: F) -> Self {
        Assignment((wire, value))
    }
}

impl<F: PrimeField> Default for Assignment<F> {
    fn default() -> Self {
        Assignment((Wire::one(), F::one()))
    }
}
