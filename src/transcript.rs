use merlin::Transcript;
use zkstd::common::{CurveAffine, FftField};

pub trait ChallengeTranscript<C: CurveAffine> {
    fn append_point(&mut self, label: &'static [u8], v: &C);

    fn append_scalar(&mut self, label: &'static [u8], v: &C::Scalar);

    fn challenge_scalar(&mut self, label: &'static [u8]) -> C::Scalar;
}

impl<C: CurveAffine> ChallengeTranscript<C> for Transcript {
    fn append_point(&mut self, label: &'static [u8], v: &C) {
        let x = C::Scalar::from(v.get_x());
        let y = C::Scalar::from(v.get_y());
        <Transcript as ChallengeTranscript<C>>::append_scalar(self, label, &x);
        <Transcript as ChallengeTranscript<C>>::append_scalar(self, label, &y);
    }

    fn append_scalar(&mut self, label: &'static [u8], v: &C::Scalar) {
        self.append_message(label, &v.to_raw_bytes())
    }

    fn challenge_scalar(&mut self, label: &'static [u8]) -> C::Scalar {
        let mut buf = [0u8; 64];
        self.challenge_bytes(label, &mut buf);
        C::Scalar::from_bytes_wide(&buf)
    }
}
