use crate::constraint::Constraint;
use crate::gadget::Gadget;
use crate::wire::Wire;

use zkstd::common::PrimeField;

pub(crate) struct Builder<F: PrimeField> {
    pointer: u64,
    r1cs: Vec<Constraint<F>>,
}

impl<F: PrimeField> Builder<F> {
    pub(crate) fn new() -> Self {
        Builder {
            pointer: 1,
            r1cs: Vec::new(),
        }
    }

    pub(crate) fn wire(&mut self) -> Wire {
        let pointer = self.pointer;
        self.pointer += 1;
        Wire::new(pointer)
    }

    pub(crate) fn equal_gate(&mut self, a: impl Into<F>, b: impl Into<F>) {
        self.add_constraint(a.into(), F::one(), b.into())
    }

    fn add_constraint(&mut self, a: F, b: F, c: F) {
        self.r1cs.push(Constraint::new(a, b, c))
    }

    pub(crate) fn build(self) -> Gadget<F> {
        let init = vec![F::zero()];
        let (mut l_c, mut r_c, mut o_c) = (init.clone(), init.clone(), init);
        self.r1cs.iter().for_each(|constraint| {
            let Constraint {
                left,
                right,
                output,
            } = constraint;
            l_c.push(*left);
            r_c.push(*right);
            o_c.push(*output);
        });
        Gadget::new(l_c, r_c, o_c)
    }
}
