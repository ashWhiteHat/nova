use zkstd::common::PrimeField;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Wire {
    pub(crate) pointor: u64,
}

impl Wire {
    pub(crate) fn new(pointor: u64) -> Self {
        Self { pointor }
    }

    pub(crate) fn one() -> Self {
        Self { pointor: 0 }
    }

    pub(crate) fn get(&self) -> u64 {
        self.pointor
    }
}
