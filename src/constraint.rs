use zkstd::common::PrimeField;

pub(crate) struct Constraint<F: PrimeField> {
    pub(crate) a: F,
    pub(crate) b: F,
    pub(crate) c: F,
}
