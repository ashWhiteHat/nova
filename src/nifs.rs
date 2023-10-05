use crate::matrix::SparseMatrix;
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
    ) -> (ProvingKey, VerificationKey) {
        let R1csStructure { m, l, a, b, c } = r1cs;
        (ProvingKey { pp, a, b, c },)
    }
}

struct ProvingKey<C: CurveAffine> {
    pp: PublicParams<C>,
    a: SparseMatrix<C::Scalar>,
    b: SparseMatrix<C::Scalar>,
    c: SparseMatrix<C::Scalar>,
}

struct VerificationKey<F: PrimeField> {
    digest: F,
}
