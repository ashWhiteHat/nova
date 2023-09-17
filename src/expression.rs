use crate::wire::Wire;

use zkstd::common::PrimeField;

pub struct Expression<F: PrimeField> {
    // sparse gate expression
    coeffs: Vec<(Wire, F)>,
}

impl<F: PrimeField> Expression<F> {
    pub(crate) fn new(coeffs: Vec<(Wire, F)>) -> Self {
        Self { coeffs }
    }
}

impl<F: PrimeField> From<Wire> for Expression<F> {
    fn from(value: Wire) -> Self {
        Self {
            coeffs: vec![(value, F::one())],
        }
    }
}
