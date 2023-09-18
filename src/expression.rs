use crate::wire::Wire;

use zkstd::common::PrimeField;

#[derive(Debug, Clone)]
pub struct Expression<F: PrimeField> {
    // sparse gate expression
    pub(crate) coeffs: Vec<(Wire, F)>,
}

impl<F: PrimeField> From<Wire> for Expression<F> {
    fn from(value: Wire) -> Self {
        Self {
            coeffs: vec![(value, F::one())],
        }
    }
}

impl<F: PrimeField> From<F> for Expression<F> {
    fn from(value: F) -> Self {
        Self {
            coeffs: vec![(Wire::one(), value)],
        }
    }
}
