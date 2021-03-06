use crate::util::i24;
use crate::AudioFormat;

#[derive(Copy, Clone, Default, Debug)]
pub struct F32;

impl AudioFormat for F32 {
    type Sample = f32;

    #[inline]
    fn from_f32(&self, s: &f32) -> f32 {
        *s
    }

    #[inline]
    fn to_bytes(&self, data: &[f32], buf: &mut Vec<u8>) {
        for sample in data {
            buf.extend(&sample.to_ne_bytes());
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct S16;

impl AudioFormat for S16 {
    type Sample = i16;

    #[inline]
    fn from_f32(&self, s: &f32) -> i16 {
        let int_value = *s * (Self::Sample::MAX as f32 + 0.5) - 0.5;

        // Casting floats to ints truncates by default, which results
        // in larger quantization error than rounding arithmetically.
        // Flooring is faster, but again with larger error.
        int_value.round() as Self::Sample
    }

    #[inline]
    fn to_bytes(&self, data: &[i16], buf: &mut Vec<u8>) {
        for sample in data {
            buf.extend(&sample.to_ne_bytes());
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct S32;

impl AudioFormat for S32 {
    type Sample = i32;

    #[inline]
    fn from_f32(&self, s: &f32) -> i32 {
        let int_value = *s * (Self::Sample::MAX as f32 + 0.5) - 0.5;
        int_value.round() as Self::Sample
    }

    #[inline]
    fn to_bytes(&self, data: &[i32], buf: &mut Vec<u8>) {
        for sample in data {
            buf.extend(&sample.to_ne_bytes());
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct S24_3;

impl AudioFormat for S24_3 {
    type Sample = i24;

    fn from_f32(&self, s: &f32) -> i24 {
        i24::pcm_from_i32(S32.from_f32(s))
    }

    fn to_bytes(&self, data: &[i24], buf: &mut Vec<u8>) {
        for sample in data {
            // The array is already in native endianness
            buf.extend(&sample.0);
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct S24_4;

impl AudioFormat for S24_4 {
    type Sample = i32;

    fn from_f32(&self, s: &f32) -> i32 {
        let int_value = *s * (Self::Sample::MAX as f32 + 0.5) - 0.5;

        // Casting floats to ints truncates by default, which results
        // in larger quantization error than rounding arithmetically.
        // Flooring is faster, but again with larger error.
        int_value.round() as Self::Sample >> 8
    }

    fn to_bytes(&self, data: &[Self::Sample], buf: &mut Vec<u8>) {
        for sample in data {
            buf.extend(&sample.to_ne_bytes());
        }
    }
}
