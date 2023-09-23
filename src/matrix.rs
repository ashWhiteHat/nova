mod elements;

use crate::wire::Wire;

pub(crate) use elements::Element;
use zkstd::common::PrimeField;

#[derive(Clone, Debug, Default)]
pub(crate) struct SparseMatrix<F: PrimeField>(pub(crate) Vec<Vec<Element<F>>>);

impl<F: PrimeField> SparseMatrix<F> {
    pub(crate) fn prod(&self, m: usize, x: &Vec<F>, w: &Vec<F>) -> Vec<F> {
        let mut vectors = vec![F::zero(); m];
        for (index, elements) in self.0.iter().enumerate() {
            vectors[index] = elements.iter().fold(F::zero(), |sum, element| {
                let (wire, coeff) = (element.0, element.1);
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
