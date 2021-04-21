#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum AudioFormatEnum {
    F32,
    I16,
    I32,
    I24_3,
    I24_4
}

impl Default for AudioFormatEnum {
    fn default() -> Self {
        Self::I16
    }
}

#[derive(Debug, Clone, Default)]
pub struct SinkConfig {
    pub device_name: Option<String>,
    pub audio_format: AudioFormatEnum,
}
