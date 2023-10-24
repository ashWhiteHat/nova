use crate::matrix::DenseVectors;
use crate::relaxed_r1cs::{Instance as RelaxedR1csInstance, Witness as RelaxedR1csWitness};

use zkstd::common::{Group, Ring, TwistedEdwardsAffine};

/// witness for r1cs (W, x, 1)
#[derive(Clone, Debug)]
pub struct Witness<C: TwistedEdwardsAffine> {
    /// intermediate value and private inputs
    pub(crate) w: DenseVectors<C::Scalar>,
    /// public inputs and outputs
    pub(crate) x: DenseVectors<C::Scalar>,
    /// first public input element one
    pub(crate) one: C::Scalar,
}

impl<C: TwistedEdwardsAffine> Default for Witness<C> {
    fn default() -> Self {
        Self {
            w: DenseVectors(vec![]),
            x: DenseVectors(vec![]),
            one: C::Scalar::one(),
        }
    }
}

impl<C: TwistedEdwardsAffine> Witness<C> {
    pub(crate) fn get(&self) -> (DenseVectors<C::Scalar>, DenseVectors<C::Scalar>) {
        (self.x.clone(), self.w.clone())
    }

    pub(crate) fn public_len(&self) -> usize {
        self.x.0.len()
    }

    pub(crate) fn private_len(&self) -> usize {
        self.w.0.len()
    }

    pub(crate) fn append_instance(&mut self, instance: C::Scalar) {
        self.x.0.push(instance)
    }

    pub(crate) fn append_witness(&mut self, witness: C::Scalar) {
        self.w.0.push(witness)
    }

    pub(crate) fn relax(&self, m: usize) -> (RelaxedR1csWitness<C>, RelaxedR1csInstance<C>) {
        let Self { w, x, one: _ } = self;
        (
            RelaxedR1csWitness {
                w: w.clone(),
                e: DenseVectors(vec![C::Scalar::zero(); m]),
            },
            RelaxedR1csInstance {
                commit_w: C::ADDITIVE_IDENTITY,
                commit_e: C::ADDITIVE_IDENTITY,
                u: C::Scalar::one(),
                x: x.clone(),
            },
        )
    }
}
