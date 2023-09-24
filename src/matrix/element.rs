use crate::wire::Wire;

use zkstd::common::{Add, Mul, PrimeField, Sub};

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

#[derive(Clone, Debug)]
pub(crate) struct DenseVectors<F: PrimeField>(pub(crate) Vec<F>);

impl<F: PrimeField> Mul<F> for DenseVectors<F> {
    type Output = Self;

    fn mul(self, rhs: F) -> Self {
        Self(self.0.iter().map(|element| *element * rhs).collect())
    }
}

impl<F: PrimeField> Mul for DenseVectors<F> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.0.len(), rhs.0.len());

        Self(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(a, b)| *a * *b)
                .collect(),
        )
    }
}

impl<F: PrimeField> Add for DenseVectors<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.0.len(), rhs.0.len());

        Self(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(a, b)| *a + *b)
                .collect(),
        )
    }
}

impl<F: PrimeField> Sub for DenseVectors<F> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.0.len(), rhs.0.len());

        Self(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(a, b)| *a - *b)
                .collect(),
        )
    }
}
