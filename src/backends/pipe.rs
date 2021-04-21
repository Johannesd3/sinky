use crate::{formats, AudioFormat, Sample, Sink, SinkConfig, SinkMaker};

use crate::config::AudioFormatEnum::*;

pub fn open<S: SinkMaker>(config: &SinkConfig, filters: S) -> S::Output {
    match config.audio_format {
        F32 => filters.apply_filters(WriteSink::<_, formats::F32>::new(std::io::stdout())),
        _ => unimplemented!("Not supported"),
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

    fn write(&mut self, data: &[Sample<Self>]) {
        self.buf.clear();
        self.format.to_bytes(data, &mut self.buf);
        self.out.write_all(&self.buf).unwrap();
    }

    fn write_mut(&mut self, data: &mut [Sample<Self>]) {
        self.write(data)
    }
}
