// Must implement Default, but that's not a problem since this trait
// is intended to be implemented for unit structs.
pub trait AudioFormat: Default {
    type Sample: Copy + Send + 'static;

    // Could be practial for use in backends?
    // const SILENCE: Self::Sample;

    // Using &self is usually a zero-cost abstraction for ZSTs, but it would
    // allow dynamical dispatch, whether we need it or not.
    fn from_f32(&self, s: &f32) -> Self::Sample;

    // In case we really would use dynamical dispatch, this function would
    // improve performance (but we don't)
    fn from_f32_buf(&self, samples: &[f32], buf: &mut Vec<Self::Sample>) {
        buf.clear();
        for s in samples {
            buf.push(self.from_f32(s));
        }
    }

    // Converts samples to bytes. Used by many backends.
    fn to_bytes(&self, data: &[Self::Sample], buf: &mut Vec<u8>);
}

// Shortcut to get the sample type of a sink
type Sample<S> = <<S as Sink>::Format as AudioFormat>::Sample;

pub trait Sink {
    type Format: AudioFormat;

    fn start(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn stop(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    // There are many further functions possible: e.g. drain, flush, set_volume

    fn write(&mut self, data: &[Sample<Self>]) {
        // Clones the data and passes it to write_mut. Could be overriden if no in-place
        // mutation is necessary.
        let mut data = data.to_vec();
        self.write_mut(&mut data);
    }

    // Would allow in-place modification.
    fn write_mut(&mut self, data: &mut [Sample<Self>]);
}

pub mod formats;

pub mod filters;

mod config;

pub use config::{AudioFormatEnum, SinkConfig};

pub trait SinkMaker {
    type Output;

    fn apply_filters<S: Sink + 'static>(&self, sink: S) -> Self::Output;
}

// This struct would usually contain the configuration for the filters.
struct ExampleSinkMaker;

impl SinkMaker for ExampleSinkMaker {
    type Output = Box<dyn Sink<Format = formats::F32>>;

    fn apply_filters<S: Sink + 'static>(&self, sink: S) -> Self::Output {
        use filters::*;

        // TODO: Use &self here to customize the Requantizer

        Box::new(SampleConverter::new(Requantizer::new(sink)))
    }
}

pub mod backends;
