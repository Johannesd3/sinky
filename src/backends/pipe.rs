use std::io::Write;

use crate::{formats, AudioFormat, AudioFormatEnum, Sample, Sink, SinkError, SinkMaker};

pub fn open<W: Write + 'static, S: SinkMaker>(
    output: W,
    format: AudioFormatEnum,
    filters: S,
) -> S::Output {
    use AudioFormatEnum::*;

    macro_rules! sink {
        ($format:path) => {
            filters.apply_filters(WriteSink::<_, $format>::new(output))
        };
    }

    match format {
        F32 => sink!(formats::F32),
        I16 => sink!(formats::I16),
        I32 => sink!(formats::I32),
        I24_3 => sink!(formats::I24_3),
        I24_4 => sink!(formats::I24_4),
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
