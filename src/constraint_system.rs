use crate::matrix::Element;
use crate::r1cs::R1csInstance;
use crate::wire::Wire;

use zkstd::common::{Ring, TwistedEdwardsAffine};

#[derive(Debug)]
pub struct ConstraintSystem<C: TwistedEdwardsAffine> {
    r1cs: R1csInstance<C>,
}

impl<C: TwistedEdwardsAffine> ConstraintSystem<C> {
    /// init constraint system with first instance one
    pub fn new() -> Self {
        Self {
            r1cs: R1csInstance::default(),
        }
    }

    /// assign instance value to constraint system
    pub fn public_wire(&mut self, instance: C::Scalar) -> Wire {
        let index = self.r1cs.witness.public_len();
        self.r1cs.witness.append_instance(instance);
        Wire::instance(index)
    }

    /// assign witness value to constraint system
    pub fn private_wire(&mut self, witness: C::Scalar) -> Wire {
        let index = self.r1cs.witness.private_len();
        self.r1cs.witness.append_witness(witness);
        Wire::witness(index)
    }

    /// constrain a + b == c
    pub fn add_constraint(&mut self, a: Wire, b: Wire, c: Wire) {
        self.r1cs.r1cs.append_a(a);
        self.enable_constraint(b, C::Scalar::one(), c)
    }

    /// constrain a * b == c
    pub fn mul_constraint(&mut self, a: Wire, b: Wire, c: Wire) {
        self.enable_constraint(a, b, c)
    }

    /// constrain a == b
    pub fn equal_constraint(&mut self, a: Wire, b: Wire) {
        self.enable_constraint(a, C::Scalar::one(), b)
    }

    /// add constraint internally
    fn enable_constraint(
        &mut self,
        a: impl Into<Element<C::Scalar>>,
        b: impl Into<Element<C::Scalar>>,
        c: impl Into<Element<C::Scalar>>,
    ) {
        self.r1cs.r1cs.append(a, b, c);
        self.r1cs.r1cs.increment()
    }

    /// check whether constraints satisfy
    pub fn is_sat(&self) -> bool {
        self.r1cs.is_sat()
    }
}

#[cfg(test)]
mod tests {
    use super::ConstraintSystem;

    use jub_jub::{Fr as Scalar, JubjubAffine as Curve};
    use zkstd::common::PrimeField;

    #[test]
    fn equal_constraint_test() {
        let x = Scalar::one().double();

        let mut cs = ConstraintSystem::<Curve>::new();
        let (a, b) = (cs.public_wire(x), cs.public_wire(x));
        cs.equal_constraint(a, b);

        assert!(cs.is_sat())
    }

    #[test]
    fn mul_constraint_test() {
        let x = Scalar::one().double();
        let y = Scalar::one().double().double();
        let z = x * y;

        let mut cs = ConstraintSystem::<Curve>::new();
        let (a, b, c) = (cs.public_wire(x), cs.public_wire(y), cs.public_wire(z));
        cs.mul_constraint(a, b, c);

        assert!(cs.is_sat())
    }

    #[test]
    fn arithmetic_constraint_test() {
        // R1CS for: x^3 + x + 5 = 35
        // https://www.vitalik.ca/general/2016/12/10/qap.html
        let x = Scalar::from(3);
        let xx = x.square();
        let y = Scalar::from(27);
        let xy = Scalar::from(30);
        let five = Scalar::from(5);
        let output = Scalar::from(35);

        let mut cs = ConstraintSystem::<Curve>::new();
        let (a, b, e, f) = (
            cs.public_wire(x),
            cs.public_wire(y),
            cs.public_wire(output),
            cs.public_wire(five),
        );
        let (c, d) = (cs.private_wire(xx), cs.private_wire(xy));

        // first: x * x = x^2
        cs.mul_constraint(a, a, c);
        // second: x^2 * x = y
        cs.mul_constraint(a, c, b);
        // third: y + x = sym2
        cs.add_constraint(b, a, d);
        // forth: sym2 + 5 = 35
        cs.add_constraint(d, f, e);

        assert!(cs.is_sat())
    }
}
