use crate::wire::Wire;

use zkstd::common::PrimeField;

#[derive(Debug, Clone)]
pub struct Expression<F: PrimeField> {
    // sparse gate expression
    coeffs: Vec<(Wire, F)>,
}

impl<F: PrimeField> Expression<F> {
    pub(crate) fn new(coeffs: Vec<(Wire, F)>) -> Self {
        Self { coeffs }
    }

    pub(crate) fn get(&self, wire: Wire) -> F {
        for (index, value) in self.coeffs.clone() {
            if index == wire {
                return value;
            }
        }
        return F::zero();
    }
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
