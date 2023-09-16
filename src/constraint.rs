use zkstd::common::PrimeField;

pub(crate) struct Constraint<F: PrimeField> {
    pub(crate) left: F,
    pub(crate) right: F,
    pub(crate) output: F,
}

impl<F: PrimeField> Constraint<F> {
    pub(crate) fn new(left: F, right: F, output: F) -> Self {
        Constraint {
            left,
            right,
            output,
        }
    }
}
