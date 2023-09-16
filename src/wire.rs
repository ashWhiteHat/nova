use zkstd::common::PrimeField;

pub(crate) struct Wire {
    pub(crate) pointor: u64,
}

impl Wire {
    pub(crate) fn new(pointor: u64) -> Self {
        Self { pointor }
    }

    pub(crate) fn get(&self) -> u64 {
        self.pointor
    }
}
