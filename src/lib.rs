mod assignment;
mod builder;
mod constraint;
mod gadget;
mod wire;

#[cfg(test)]
mod tests {
    use crate::assignment::Assignment;
    use crate::builder::Builder;
    use bls_12_381::Fr as BlsScalar;

    #[test]
    fn it_works() {
        let mut builder = Builder::<BlsScalar>::new();
        let (a, b) = (builder.wire(), builder.wire());
        builder.equal_gate(a.get(), b.get());
        let gadget = builder.build();

        let x = BlsScalar::one();
        let assignments = vec![Assignment::new(a, x), Assignment::new(b, x)];
        let is_sat = gadget.is_sat(assignments);
        assert!(is_sat)
    }
}
