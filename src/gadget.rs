use crate::assignment::Assignment;
use crate::constraint::Constraint;

use zkstd::common::PrimeField;

pub(crate) struct Gadget<F: PrimeField> {
    pub(crate) left_coeffs: Vec<F>,
    pub(crate) right_coeffs: Vec<F>,
    pub(crate) output_coeffs: Vec<F>,
}

impl<F: PrimeField> Gadget<F> {
    pub(crate) fn new(left_coeffs: Vec<F>, right_coeffs: Vec<F>, output_coeffs: Vec<F>) -> Self {
        Self {
            left_coeffs,
            right_coeffs,
            output_coeffs,
        }
    }

    pub(crate) fn is_sat(&self, assignments: Vec<Assignment<F>>) -> bool {
        let Gadget {
            left_coeffs,
            right_coeffs,
            output_coeffs,
        } = self;
        let left_product = dot_product(left_coeffs, &assignments);
        let right_product = dot_product(right_coeffs, &assignments);
        let output_product = dot_product(output_coeffs, &assignments);
        left_product * right_product == output_product
    }
}

fn dot_product<F: PrimeField>(coeffs: &Vec<F>, witnessess: &Vec<Assignment<F>>) -> F {
    witnessess.iter().fold(F::zero(), |sum, assignment| {
        let (wire, value) = &assignment.0;
        sum + coeffs[wire.get() as usize] * value
    })
}
