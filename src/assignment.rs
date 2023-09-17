use crate::wire::Wire;

use zkstd::common::PrimeField;

#[derive(Debug)]
pub struct Assignment<F: PrimeField>(pub(crate) (Wire, F));

impl<F: PrimeField> Assignment<F> {
    pub fn new(wire: Wire, value: F) -> Self {
        Assignment((wire, value))
    }
}
