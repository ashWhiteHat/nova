use crate::prover::Prover;
use crate::public_param::PedersenCommitment;
use crate::r1cs::R1csStructure;

use zkstd::common::{CurveAffine, PrimeField, RngCore};

struct Nifs<C: CurveAffine> {
    pp: PedersenCommitment<C>,
}

impl<C: CurveAffine> Nifs<C> {
    pub(crate) fn g(λ: u64, r: impl RngCore) -> PedersenCommitment<C> {
        PedersenCommitment::new(λ, r)
    }

    pub(crate) fn k(
        pp: PedersenCommitment<C>,
        r1cs: R1csStructure<C::Scalar>,
    ) -> (Prover<C>, VerificationKey<C::Scalar>) {
        let digest = pp.digest();
        (Prover { pp, f: r1cs, i: 0 }, VerificationKey { digest })
    }
}

struct VerificationKey<F: PrimeField> {
    digest: F,
}
