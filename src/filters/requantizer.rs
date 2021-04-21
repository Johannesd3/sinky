use super::SampleMapper;
use crate::AudioFormat;

#[derive(Default)]
pub struct Requantizer;

impl<F: AudioFormat> SampleMapper<F> for Requantizer {
    type Output = F;

    fn map(&self, sample: &F::Sample) -> F::Sample {
        // TODO: do stuff

        *sample
    }
}
