use zkstd::common::CurveAffine;

pub trait Transcript<C: CurveAffine> {
    fn absorb_point(&mut self, v: &C);
    fn get_challenge(&mut self) -> C::Scalar;
}
