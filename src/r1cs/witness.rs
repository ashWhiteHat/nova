use zkstd::common::PrimeField;

#[derive(Debug, Default)]
pub(crate) struct R1csWitness<F: PrimeField> {
    pub(crate) x: Vec<F>,
    pub(crate) w: Vec<F>,
}

impl<F: PrimeField> R1csWitness<F> {
    pub(crate) fn public_len(&self) -> usize {
        self.x.len()
    }

    pub(crate) fn private_len(&self) -> usize {
        self.w.len()
    }

    pub(crate) fn append_instance(&mut self, instance: F) {
        self.x.push(instance)
    }

    pub(crate) fn append_witness(&mut self, witness: F) {
        self.w.push(witness)
    }
}
