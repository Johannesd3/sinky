mod convert;
pub use convert::SampleConverter;

mod volume;
pub use volume::Volume;

use crate::{AudioFormat, Sample, Sink};

pub trait SampleMapper<F: AudioFormat> {
    type Output: AudioFormat;

    fn map(&self, sample: &F::Sample) -> <Self::Output as AudioFormat>::Sample;
}

pub trait Filter<F: AudioFormat> {
    fn filter(&self, data: &mut [F::Sample]);
}

impl<A: AudioFormat, F: SampleMapper<A, Output = A>> Filter<A> for F {
    fn filter(&self, data: &mut [A::Sample]) {
        for sample in data {
            *sample = self.map(sample);
        }
    }
}

pub trait Converter<F: AudioFormat> {
    type Output: AudioFormat;

    fn convert(&self, data: &[F::Sample], buf: &mut Vec<<Self::Output as AudioFormat>::Sample>);
}

impl<A: AudioFormat, F: SampleMapper<A>> Converter<A> for F {
    type Output = F::Output;

    fn convert(&self, data: &[A::Sample], buf: &mut Vec<<Self::Output as AudioFormat>::Sample>) {
        buf.reserve(data.len());

        for sample in data {
            buf.push(self.map(sample))
        }
    }
}

#[doc(hidden)]
pub struct FilterSink<F: Filter<S::Format>, S: Sink> {
    filter: F,
    sink: S,
}

impl<F: Filter<S::Format>, S: Sink> Sink for FilterSink<F, S> {
    type Format = S::Format;

    fn start(&mut self) -> Result<(), crate::SinkError> {
        self.sink.start()
    }

    fn stop(&mut self) -> Result<(), crate::SinkError> {
        self.sink.stop()
    }

    fn write(&mut self, data: &mut [crate::Sample<Self>]) -> Result<(), crate::SinkError> {
        self.filter.filter(data);
        self.sink.write(data)
    }
}

#[doc(hidden)]
pub struct ConverterSink<A: AudioFormat, F: Converter<A>, S: Sink<Format = F::Output>> {
    _format: A,
    filter: F,
    sink: S,
    buf: Vec<Sample<S>>,
}

impl<A: AudioFormat, F: Converter<A>, S: Sink<Format = F::Output>> Sink for ConverterSink<A, F, S> {
    type Format = A;

    fn start(&mut self) -> Result<(), crate::SinkError> {
        self.sink.start()
    }

    fn stop(&mut self) -> Result<(), crate::SinkError> {
        self.sink.stop()
    }

    fn write(&mut self, data: &mut [Sample<Self>]) -> Result<(), crate::SinkError> {
        self.buf.clear();
        self.filter.convert(data, &mut self.buf);
        self.sink.write(&mut self.buf)
    }
}

pub trait SinkExt<'a>: Sink + Sized + 'a {
    fn add_converter<F: AudioFormat, C: Converter<F, Output = Self::Format>>(
        self,
        converter: C,
    ) -> ConverterSink<F, C, Self> {
        ConverterSink {
            _format: Default::default(),
            filter: converter,
            buf: Vec::new(),
            sink: self,
        }
    }

    fn add_filter<F: Filter<Self::Format>>(self, filter: F) -> FilterSink<F, Self> {
        FilterSink { filter, sink: self }
    }

    fn boxed(self) -> Box<dyn Sink<Format = Self::Format> + 'a> {
        Box::new(self)
    }
}

impl<'a, S: Sink + 'a> SinkExt<'a> for S {}
