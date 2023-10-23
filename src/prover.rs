use crate::committed_relaxed_r1cs::{
    CommittedRelaxedR1csInstance, Instance as CommittedRelaxedR1csInstanceData,
    Witness as CommittedRelaxedR1csWitness,
};
use crate::matrix::DenseVectors;
use crate::proof::IvcProof;
use crate::public_param::PedersenCommitment;
use crate::r1cs::{Instance as R1csInstance, R1csStructure, Witness as R1csWitness};
use crate::relaxed_r1cs::{Instance as RelaxedR1csInstance, Witness as RelaxedR1csWitness};

use zkstd::common::{CurveAffine, PrimeField, Ring};

pub(crate) struct Prover<C: CurveAffine> {
    pub(crate) pp: PedersenCommitment<C>,
    pub(crate) f: R1csStructure<C::Scalar>,
    pub(crate) i: usize,
}

impl<C: CurveAffine> Prover<C> {
    pub(crate) fn new(pp: PedersenCommitment<C>, f: R1csStructure<C::Scalar>) -> Self {
        Self { pp, f, i: 0 }
    }

    pub(crate) fn recurse(
        &self,
        z0: Vec<C::Scalar>,
        zi: Vec<C::Scalar>,
        πi: IvcProof<C>,
    ) -> IvcProof<C> {
        if self.i == 0 {}
        πi
    }
}

#[cfg(test)]
mod tests {
    use crate::relaxed_r1cs::Witness as RelaxedR1csWitness;
    use crate::tests::example_r1cs;

    use jub_jub::Fr as Scalar;

    #[test]
    fn folding_test() {
        let r1cs = example_r1cs::<Scalar>();
        let w0 = RelaxedR1csWitness::init(r1cs);
    }
}
