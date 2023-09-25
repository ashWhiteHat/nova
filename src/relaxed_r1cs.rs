mod blueprint;
mod witness;

pub(crate) use blueprint::RelaxedR1CS;
pub(crate) use witness::RelaxedR1csWitness;
use zkstd::common::PrimeField;

pub(crate) struct RelaxedR1CSInstance<F: PrimeField> {
    pub(crate) relaxed_r1cs: RelaxedR1CS<F>,
    pub(crate) relaxed_z: RelaxedR1csWitness<F>,
}
