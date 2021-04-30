use std::io::Write;

use crate::{formats, AudioFormat, AudioFormatConfig, Sample, Sink, SinkError, SinkMaker};

pub fn open<W: Write + 'static, S: SinkMaker>(
    output: W,
    format: AudioFormatConfig,
    filters: S,
) -> S::Output {
    use AudioFormatConfig::*;

    macro_rules! sink {
        ($format:path) => {
            filters.apply_filters(WriteSink::<_, $format>::new(output))
        };
    }

    match format {
        F32 => sink!(formats::F32),
        S16 => sink!(formats::S16),
        S32 => sink!(formats::S32),
        S24_3 => sink!(formats::S24_3),
        S24_4 => sink!(formats::S24_4),
    }
}

struct WriteSink<W: std::io::Write, F: AudioFormat> {
    out: W,
    buf: Vec<u8>,
    format: F, // Usually zero-sized
}

impl<W: std::io::Write, F: AudioFormat + Default> WriteSink<W, F> {
    pub fn new(out: W) -> Self {
        Self {
            out,
            buf: Vec::new(),
            format: Default::default(),
        }
    }
}

impl<W: std::io::Write, F: AudioFormat> Sink for WriteSink<W, F> {
    type Format = F;

    fn write(&mut self, data: &mut [Sample<Self>]) -> Result<(), SinkError> {
        self.buf.clear();
        self.format.to_bytes(data, &mut self.buf);
        self.out
            .write_all(&self.buf)
            .map_err(|e| SinkError(Box::new(e)))
    }
}
