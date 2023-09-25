mod blueprint;
mod witness;

pub(crate) use blueprint::R1cs;
pub(crate) use witness::R1csWitness;

use zkstd::common::PrimeField;

pub(crate) struct R1csInstance<F: PrimeField> {
    pub(crate) r1cs: R1cs<F>,
    pub(crate) z: R1csWitness<F>,
}

impl<F: PrimeField> R1csInstance<F> {
    pub(crate) fn new(r1cs: &R1cs<F>, z: &Vec<F>) -> Self {
        let z = r1cs.instance_and_witness(z.to_vec());
        let r1cs = r1cs.clone();
        Self { r1cs, z }
    }
}
