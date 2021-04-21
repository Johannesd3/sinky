
use super::AudioFormat;

#[derive(Copy, Clone, Default, Debug)]
pub struct F32;

impl AudioFormat for F32 {
    type Sample = f32;

    fn from_f32(&self, s: &f32) -> f32 {
        *s
    }

    fn to_bytes(&self, data: &[f32], buf: &mut Vec<u8>) {
        todo!()
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct I16;

#[derive(Copy, Clone, Default, Debug)]
pub struct I32;

#[derive(Copy, Clone, Default, Debug)]
pub struct I24_3;

#[derive(Copy, Clone, Default, Debug)]
pub struct I24_4;

impl AudioFormat for I16 {
    type Sample = i16;

    fn from_f32(&self, s: &f32) -> i16 {
        todo!()
    }

    fn to_bytes(&self, data: &[i16], buf: &mut Vec<u8>) {
        todo!()
    }
}

// ...and so on
