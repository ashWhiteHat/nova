mod assignment;
mod constraint;
mod constraint_system;
mod expression;
mod gadget;
mod wire;

pub use assignment::Assignment;
pub use constraint_system::ConstraintSystem;

#[cfg(test)]
mod tests {
    use super::{Assignment, ConstraintSystem};

    use bls_12_381::Fr as BlsScalar;
    use zkstd::common::PrimeField;

    #[test]
    fn equal_gate_test() {
        let x = BlsScalar::one().double();

        let mut cs = ConstraintSystem::<BlsScalar>::new();
        let (a, b) = (cs.public_wire(x), cs.public_wire(x));
        cs.equal_constraint(a, b);
        let gadget = cs.build();

        let assignments = vec![
            Assignment::new(a, x),
            Assignment::new(b, x),
            Assignment::default(),
        ];
        let is_sat = gadget.is_sat(assignments);
        assert!(is_sat)
    }

    #[test]
    fn mul_gate_test() {
        let x = BlsScalar::one().double();
        let y = BlsScalar::one().double().double();
        let z = x * y;

        let mut cs = ConstraintSystem::<BlsScalar>::new();
        let (a, b, c) = (cs.public_wire(x), cs.public_wire(y), cs.public_wire(z));
        cs.mul_constraint(a, b, c);
        let gadget = cs.build();

        let assignments = vec![
            Assignment::new(a, x),
            Assignment::new(b, y),
            Assignment::new(c, z),
            Assignment::default(),
        ];
        let is_sat = gadget.is_sat(assignments);
        assert!(is_sat)
    }
}
