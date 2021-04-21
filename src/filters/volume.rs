use super::SampleMapper;
use crate::formats::F32;

pub struct Volume(pub f32);

impl SampleMapper<F32> for Volume {
    type Output = F32;

    fn map(&self, sample: &f32) -> f32 {
        *sample * self.0
    }
}
