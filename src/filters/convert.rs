use crate::{formats, AudioFormat, Sample, Sink};

pub struct SampleConverter<S: Sink> {
    sink: S,
    buf: Vec<Sample<S>>,
    format: S::Format,
}

impl<S: Sink> SampleConverter<S> {
    pub fn new(sink: S) -> Self {
        Self {
            sink,
            buf: Vec::new(),
            format: Default::default(),
        }
    }
}

impl<S: Sink> Sink for SampleConverter<S> {
    type Format = formats::F32;

    fn start(&mut self) -> std::io::Result<()> {
        self.sink.start()
    }

    fn stop(&mut self) -> std::io::Result<()> {
        self.sink.stop()
    }

    fn write(&mut self, data: &[f32]) {
        self.format.from_f32_buf(data, &mut self.buf);
        self.sink.write_mut(&mut self.buf);
    }

    fn write_mut(&mut self, data: &mut [f32]) {
        self.write(data);
    }
}
