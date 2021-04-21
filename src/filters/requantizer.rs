use crate::{Sample, Sink};

pub struct Requantizer<S: Sink> {
    sink: S,
}

impl<S: Sink> Requantizer<S> {
    pub fn new(sink: S) -> Self {
        Self { sink }
    }
}

impl<S: Sink> Sink for Requantizer<S> {
    type Format = S::Format;

    fn start(&mut self) -> std::io::Result<()> {
        self.sink.start()
    }

    fn stop(&mut self) -> std::io::Result<()> {
        self.sink.stop()
    }

    fn write_mut(&mut self, data: &mut [Sample<Self>]) {
        todo!();
        self.sink.write_mut(data);
    }
}
