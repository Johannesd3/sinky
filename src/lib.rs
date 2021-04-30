use std::fmt;

// Must implement Default, but that's not a problem since this trait
// is intended to be implemented for unit structs.
pub trait AudioFormat: Default {
    type Sample: Copy + Send + 'static;

    // Using &self is usually a zero-cost abstraction for ZSTs, but it would
    // allow dynamical dispatch, whether we need it or not.
    fn from_f32(&self, s: &f32) -> Self::Sample;

    // Converts samples to bytes. Used by many backends.
    fn to_bytes(&self, data: &[Self::Sample], buf: &mut Vec<u8>);
}

// Shortcut to get the sample type of a sink
type Sample<S> = <<S as Sink>::Format as AudioFormat>::Sample;

#[derive(Debug)]
pub struct SinkError(Box<dyn std::error::Error + 'static>);

impl SinkError {
    pub fn new<E: std::error::Error + 'static>(error: E) -> Self {
        SinkError(Box::new(error))
    }
}

impl fmt::Display for SinkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for SinkError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.0.as_ref())
    }
}

pub trait Sink {
    type Format: AudioFormat;

    fn start(&mut self) -> Result<(), SinkError> {
        Ok(())
    }

    fn stop(&mut self) -> Result<(), SinkError> {
        Ok(())
    }

    // Allows in-place modification.
    fn write(&mut self, data: &mut [Sample<Self>]) -> Result<(), SinkError>;

    // There are many further functions possible: e.g. drain, flush, set_volume
}

// Implement `Sink` for every boxed Sink trait object.
impl<F: AudioFormat> Sink for Box<dyn Sink<Format = F>> {
    type Format = F;

    fn start(&mut self) -> Result<(), SinkError> {
        self.as_mut().start()
    }

    fn stop(&mut self) -> Result<(), SinkError> {
        self.as_mut().stop()
    }

    fn write(&mut self, data: &mut [Sample<Self>]) -> Result<(), SinkError> {
        self.as_mut().write(data)
    }
}

pub mod formats;

pub mod filters;

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum AudioFormatConfig {
    F32,
    S16,
    S32,
    S24_3,
    S24_4,
}

impl Default for AudioFormatConfig {
    fn default() -> Self {
        Self::S16
    }
}

pub trait SinkMaker {
    type Output;

    fn apply_filters<S: Sink + 'static>(self, sink: S) -> Self::Output;
}

pub mod backends;
