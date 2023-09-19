use crate::wire::Wire;

use zkstd::common::PrimeField;

#[derive(Debug, Default)]
pub(crate) struct R1cs<F: PrimeField>(pub(crate) Vec<Constraint<F>>);

#[derive(Debug, Clone)]
pub(crate) struct Constraint<F: PrimeField> {
    pub(crate) left: Expression<F>,
    pub(crate) right: Expression<F>,
    pub(crate) output: Expression<F>,
}

impl<F: PrimeField> Constraint<F> {
    pub(crate) fn new(left: Expression<F>, right: Expression<F>, output: Expression<F>) -> Self {
        Constraint {
            left,
            right,
            output,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Expression<F: PrimeField>(pub(crate) Vec<(Wire, F)>);

impl<F: PrimeField> From<Wire> for Expression<F> {
    fn from(value: Wire) -> Self {
        Self(vec![(value, F::one())])
    }
}

impl<F: PrimeField> From<F> for Expression<F> {
    fn from(value: F) -> Self {
        Self(vec![(Wire::one(), value)])
    }
}
