mod constraint_system;
mod r1cs;
mod wire;

pub use constraint_system::ConstraintSystem;

#[cfg(test)]
mod tests {
    use super::ConstraintSystem;

    use bls_12_381::Fr as Scalar;
    use zkstd::common::PrimeField;

    #[test]
    fn equal_constraint_test() {
        let x = Scalar::one().double();

        let mut cs = ConstraintSystem::<Scalar>::new();
        let (a, b) = (cs.public_wire(x), cs.public_wire(x));
        cs.equal_constraint(a, b);

        assert!(cs.is_sat())
    }

    #[test]
    fn mul_constraint_test() {
        let x = Scalar::one().double();
        let y = Scalar::one().double().double();
        let z = x * y;

        let mut cs = ConstraintSystem::<Scalar>::new();
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

        let mut cs = ConstraintSystem::<Scalar>::new();
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
