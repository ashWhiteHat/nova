pub(crate) struct Wire {
    pub(crate) pointor: usize,
}

impl Wire {
    pub(crate) fn new(pointor: usize) -> Self {
        Self { pointor }
    }
}
