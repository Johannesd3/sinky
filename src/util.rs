#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct i24(pub(crate) [u8; 3]);
impl i24 {
    #[inline]
    pub(crate) fn pcm_from_i32(sample: i32) -> Self {
        // drop the least significant byte
        let [a, b, c, _d] = (sample >> 8).to_ne_bytes();
        i24([a, b, c])
    }
}
