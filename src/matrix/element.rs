use crate::wire::Wire;

use zkstd::common::{Mul, PrimeField};

#[derive(Clone, Debug)]
pub(crate) struct Element<F: PrimeField>(pub(crate) Wire, pub(crate) F);

impl<F: PrimeField> From<Wire> for Element<F> {
    fn from(value: Wire) -> Self {
        Self(value, F::one())
    }
}

impl<F: PrimeField> From<F> for Element<F> {
    fn from(value: F) -> Self {
        Self(Wire::one(), value)
    }
}

pub(crate) struct DenseVectors<F: PrimeField>(pub(crate) Vec<F>);

impl<F: PrimeField> Mul<F> for DenseVectors<F> {
    type Output = Self;

    fn mul(self, rhs: F) -> Self {
        Self(self.0.iter().map(|element| *element * rhs).collect())
    }
}
