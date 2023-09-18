#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wire {
    Instance(usize),
    Witness(usize),
}

impl Wire {
    pub(crate) fn public(index: usize) -> Self {
        Self::Instance(index)
    }

    pub(crate) fn private(index: usize) -> Self {
        Self::Witness(index)
    }

    pub(crate) fn one() -> Self {
        Self::Instance(0)
    }
}
