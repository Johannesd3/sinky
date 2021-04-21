use super::AudioFormat;

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
            buf.extend(&sample.to_be_bytes());
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct I16;

impl AudioFormat for I16 {
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
            buf.extend(&sample.to_be_bytes());
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct I32;

impl AudioFormat for I32 {
    type Sample = i32;

    #[inline]
    fn from_f32(&self, s: &f32) -> i32 {
        let int_value = *s * (Self::Sample::MAX as f32 + 0.5) - 0.5;
        int_value.round() as Self::Sample
    }

    #[inline]
    fn to_bytes(&self, data: &[i32], buf: &mut Vec<u8>) {
        for sample in data {
            buf.extend(&sample.to_be_bytes());
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct i24([u8; 3]);
impl i24 {
    #[inline]
    fn pcm_from_i32(sample: i32) -> Self {
        // drop the least significant byte
        let [a, b, c, _d] = (sample >> 8).to_le_bytes();
        i24([a, b, c])
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct I24_3;

impl AudioFormat for I24_3 {
    type Sample = i24;

    fn from_f32(&self, s: &f32) -> i24 {
        i24::pcm_from_i32(I32.from_f32(s))
    }

    fn to_bytes(&self, data: &[i24], buf: &mut Vec<u8>) {
        for sample in data {
            buf.extend(&sample.0);
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct I24_4;

impl AudioFormat for I24_4 {
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
            buf.extend(&sample.to_be_bytes());
        }
    }
}
