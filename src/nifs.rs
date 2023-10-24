use crate::prover::Prover;
use crate::public_param::PedersenCommitment;
use crate::r1cs::R1csStructure;

use zkstd::common::{PrimeField, RngCore, TwistedEdwardsAffine};

struct Nifs<C: TwistedEdwardsAffine> {
    pp: PedersenCommitment<C>,
}

impl<C: TwistedEdwardsAffine> Nifs<C> {
    pub(crate) fn g(λ: u64, r: impl RngCore) -> PedersenCommitment<C> {
        PedersenCommitment::new(λ, r)
    }

    pub(crate) fn k(
        pp: PedersenCommitment<C>,
        r1cs: R1csStructure<C>,
    ) -> (Prover<C>, VerificationKey<C::Scalar>) {
        let digest = pp.digest();
        (Prover { pp, f: r1cs, i: 0 }, VerificationKey { digest })
    }
}

struct VerificationKey<F: PrimeField> {
    digest: F,
}
