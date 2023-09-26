use crate::matrix::DenseVectors;
use crate::relaxed_r1cs::{RelaxedR1csInstanceData, RelaxedR1csWitness};

use zkstd::common::PrimeField;

/// witness for r1cs
#[derive(Debug)]
pub struct R1csWitness<F: PrimeField> {
    /// intermediate value and private inputs
    pub(crate) w: DenseVectors<F>,
    /// public inputs and outputs
    pub(crate) x: DenseVectors<F>,
    /// first public input element one
    pub(crate) one: F,
}

impl<F: PrimeField> Default for R1csWitness<F> {
    fn default() -> Self {
        Self {
            w: DenseVectors(vec![]),
            x: DenseVectors(vec![]),
            one: F::one(),
        }
    }
}

impl<F: PrimeField> R1csWitness<F> {
    pub(crate) fn new(x: Vec<F>, w: Vec<F>) -> Self {
        Self {
            w: DenseVectors(w),
            x: DenseVectors(x),
            one: F::one(),
        }
    }

    pub(crate) fn get(&self) -> (DenseVectors<F>, DenseVectors<F>) {
        (self.x.clone(), self.w.clone())
    }

    pub(crate) fn public_len(&self) -> usize {
        self.x.0.len()
    }

    pub(crate) fn private_len(&self) -> usize {
        self.w.0.len()
    }

    pub(crate) fn append_instance(&mut self, instance: F) {
        self.x.0.push(instance)
    }

    pub(crate) fn append_witness(&mut self, witness: F) {
        self.w.0.push(witness)
    }

    pub(crate) fn relax(&self, m: usize) -> (RelaxedR1csWitness<F>, RelaxedR1csInstanceData<F>) {
        let Self { w, x, one: _ } = self;
        let e = DenseVectors(vec![F::zero(); m]);
        let u = F::one();
        let x = x.clone();
        (
            RelaxedR1csWitness {
                w: w.clone(),
                x: x.clone(),
                u,
            },
            RelaxedR1csInstanceData { e, u, x },
        )
    }
}
