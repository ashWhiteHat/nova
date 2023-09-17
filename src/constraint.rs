use crate::expression::Expression;

use zkstd::common::PrimeField;

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
