use zkstd::common::PrimeField;

pub(crate) struct Constraint<F: PrimeField> {
    pub(crate) a: F,
    pub(crate) b: F,
    pub(crate) c: F,
}

impl<F: PrimeField> Constraint<F> {
    pub(crate) fn new(a: F, b: F, c: F) -> Self {
        Constraint { a, b, c }
    }
}
