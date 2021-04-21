use super::SampleMapper;
use crate::{formats, AudioFormat};

#[derive(Default)]
pub struct SampleConverter<F: AudioFormat>(F);

impl<F: AudioFormat> SampleMapper<formats::F32> for SampleConverter<F> {
    type Output = F;

    fn map(&self, sample: &f32) -> F::Sample {
        self.0.from_f32(sample)
    }
}
