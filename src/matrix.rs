mod element;

use crate::wire::Wire;

pub(crate) use element::{DenseVectors, Element};
use std::ops::{Index, IndexMut};
use zkstd::common::PrimeField;

#[derive(Clone, Debug, Default)]
pub(crate) struct SparseMatrix<F: PrimeField>(pub(crate) Vec<Vec<Element<F>>>);

impl<F: PrimeField> SparseMatrix<F> {
    pub(crate) fn prod(
        &self,
        m: usize,
        x: &DenseVectors<F>,
        w: &DenseVectors<F>,
    ) -> DenseVectors<F> {
        let mut vectors = DenseVectors(vec![F::zero(); m]);
        for (index, elements) in self.0.iter().enumerate() {
            vectors[index] = elements.iter().fold(F::zero(), |sum, element| {
                let (wire, coeff) = element.get();
                let value = match wire {
                    Wire::Instance(i) => x[i],
                    Wire::Witness(i) => w[i],
                };
                sum + coeff * value
            })
        }
        vectors
    }
}

impl<F: PrimeField> Index<usize> for SparseMatrix<F> {
    type Output = Vec<Element<F>>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<F: PrimeField> IndexMut<usize> for SparseMatrix<F> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
