mod constraint;
mod constraint_system;
mod expression;
mod wire;

pub use constraint_system::ConstraintSystem;

#[cfg(test)]
mod tests {
    use super::ConstraintSystem;

    use bls_12_381::Fr as BlsScalar;
    use zkstd::common::PrimeField;

    #[test]
    fn equal_constraint_test() {
        let x = BlsScalar::one().double();

        let mut cs = ConstraintSystem::<BlsScalar>::new();
        let (a, b) = (cs.public_wire(x), cs.public_wire(x));
        cs.equal_constraint(a, b);

        assert!(cs.is_sat())
    }

    #[test]
    fn mul_constraint_test() {
        let x = BlsScalar::one().double();
        let y = BlsScalar::one().double().double();
        let z = x * y;

        let mut cs = ConstraintSystem::<BlsScalar>::new();
        let (a, b, c) = (cs.public_wire(x), cs.public_wire(y), cs.public_wire(z));
        cs.mul_constraint(a, b, c);

        assert!(cs.is_sat())
    }
}
