mod assignment;
mod builder;
mod constraint;
mod expression;
mod gadget;
mod wire;

pub use assignment::Assignment;
pub use builder::Builder;

#[cfg(test)]
mod tests {
    use super::{Assignment, Builder};

    use bls_12_381::Fr as BlsScalar;
    use zkstd::common::PrimeField;

    #[test]
    fn equal_gate_test() {
        let mut builder = Builder::<BlsScalar>::new();
        let (a, b) = (builder.wire(), builder.wire());
        builder.equal_gate(a, b);
        let gadget = builder.build();

        let x = BlsScalar::one().double();
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
        let mut builder = Builder::<BlsScalar>::new();
        let (a, b, c) = (builder.wire(), builder.wire(), builder.wire());
        builder.mul_gate(a, b, c);
        let gadget = builder.build();

        let x = BlsScalar::one().double();
        let y = BlsScalar::one().double().double();
        let z = x * y;
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
