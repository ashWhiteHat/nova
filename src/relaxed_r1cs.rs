mod blueprint;
mod witness;

pub(crate) use blueprint::{commit_relaxed_r1cs, RelaxedR1CS};
pub(crate) use witness::{commit_relaxed_z, RelaxedR1csWitness};

use crate::commitment::CommitmentScheme;
use crate::committed_relaxed_r1cs::CommittedRelaxedR1CSInstance;

use zkstd::common::{CurveAffine, PrimeField};

pub(crate) struct RelaxedR1CSInstance<F: PrimeField> {
    pub(crate) relaxed_r1cs: RelaxedR1CS<F>,
    pub(crate) relaxed_z: RelaxedR1csWitness<F>,
}

pub(crate) fn commit_relaxed_r1cs_instance<C: CurveAffine>(
    relaxed_r1cs_instance: RelaxedR1CSInstance<C::Scalar>,
    r_e: C::Scalar,
    r_w: C::Scalar,
    cs: &CommitmentScheme<C>,
) -> CommittedRelaxedR1CSInstance<C> {
    let RelaxedR1CSInstance {
        relaxed_r1cs,
        relaxed_z,
    } = relaxed_r1cs_instance;
    let e = relaxed_r1cs.e.clone();
    let RelaxedR1csWitness { x, w, u: _ } = &relaxed_z;
    let committed_relaxed_r1cs = commit_relaxed_r1cs(&relaxed_r1cs, x, w, cs);
    let committed_relaxed_z = commit_relaxed_z(&relaxed_z, e, r_e, r_w);
    CommittedRelaxedR1CSInstance {
        committed_relaxed_r1cs,
        committed_relaxed_z,
    }
}
