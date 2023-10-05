use crate::prover::ProvingKey;
use crate::public_param::PublicParams;
use crate::r1cs::R1csStructure;

use zkstd::common::{CurveAffine, PrimeField, RngCore};

struct Nifs<C: CurveAffine> {
    pp: PublicParams<C>,
}

impl<C: CurveAffine> Nifs<C> {
    pub(crate) fn g(λ: u64, r: impl RngCore) -> PublicParams<C> {
        PublicParams::new(λ, r)
    }

    pub(crate) fn k(
        pp: PublicParams<C>,
        r1cs: R1csStructure<C::Scalar>,
    ) -> (ProvingKey<C>, VerificationKey<C::Scalar>) {
        let R1csStructure {
            m: _,
            l: _,
            a,
            b,
            c,
        } = r1cs;
        let digest = pp.digest();
        (ProvingKey { pp, a, b, c }, VerificationKey { digest })
    }
}

struct VerificationKey<F: PrimeField> {
    digest: F,
}
