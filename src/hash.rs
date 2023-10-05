use blake2b_simd::{Params, State};
use zkstd::common::FftField;

const PERSONAL: &[u8; 16] = b"Nova_foldingHash";

pub(crate) struct Digest(State);

impl Default for Digest {
    fn default() -> Self {
        let state = Params::new().hash_length(64).personal(PERSONAL).to_state();

        Self(state)
    }
}
impl Digest {
    pub(crate) fn update(&mut self, bytes: &[u8]) {
        self.0.update(bytes);
    }

    pub(crate) fn finalize<F: FftField>(&self) -> F {
        let digest = self.0.finalize();
        F::from_hash(digest.as_array())
    }
}
