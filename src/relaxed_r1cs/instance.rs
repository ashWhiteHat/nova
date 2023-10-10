use crate::committed_relaxed_r1cs::Instance as CommittedRelaxedR1csInstance;
use crate::matrix::DenseVectors;
use crate::public_param::PedersenCommitment;

use zkstd::common::{CurveAffine, PrimeField};

/// instance for relaxed r1cs (E, u, x)
#[derive(Clone, Debug)]
pub struct Instance<F: PrimeField> {
    /// error vectors
    pub(crate) e: DenseVectors<F>,
    /// scalar
    pub(crate) u: F,
    /// public inputs and outputs
    pub(crate) x: DenseVectors<F>,
}

pub(crate) fn commit_relaxed_r1cs_instance_data<C: CurveAffine>(
    relaxed_r1cs_instance: &Instance<C::Scalar>,
    w: &DenseVectors<C::Scalar>,
    cs: &PedersenCommitment<C>,
) -> CommittedRelaxedR1csInstance<C> {
    let Instance { e, u, x } = relaxed_r1cs_instance;
    CommittedRelaxedR1csInstance {
        overline_e: cs.commit(e, u),
        u: *u,
        overline_w: cs.commit(w, u),
        x: x.clone(),
    }
}
