use crate::constraint::Constraint;
use crate::expression::Expression;
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

    pub(crate) fn equal_gate(&mut self, a: impl Into<Expression<F>>, b: impl Into<Expression<F>>) {
        self.add_constraint(a.into(), F::one().into(), b.into())
    }

    fn add_constraint(&mut self, a: Expression<F>, b: Expression<F>, c: Expression<F>) {
        self.r1cs.push(Constraint::new(a, b, c))
    }

    pub(crate) fn build(&self) -> Gadget<F> {
        let Constraint {
            left,
            right,
            output,
        } = self.r1cs[0].clone();
        Gadget::new(left, right, output)
    }
}
