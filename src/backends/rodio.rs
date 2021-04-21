use std::thread;

use crate::formats::{F32, I16};
use crate::{AudioFormat, AudioFormatEnum, Sink, SinkError, SinkMaker};

pub fn open<S: SinkMaker>(format: AudioFormatEnum, filters: S) -> S::Output {
    macro_rules! sink {
        ($format:path) => {
            filters.apply_filters(RodioSink::<$format>::open(cpal::default_host(), None))
        };
    }

    match format {
        AudioFormatEnum::F32 => sink!(F32),
        AudioFormatEnum::I16 => sink!(I16),
        _ => unimplemented!("Abc"),
    }
}

use cpal::traits::{DeviceTrait, HostTrait};
use thiserror::Error;

trait SupportedFormat: AudioFormat
where
    Self::Sample: rodio::Sample,
{
}
impl SupportedFormat for I16 {}
impl SupportedFormat for F32 {}

#[derive(Debug, Error)]
pub enum RodioError {
    #[error("Rodio: no device available")]
    NoDeviceAvailable,
    #[error("Rodio: device \"{0}\" is not available")]
    DeviceNotAvailable(String),
    #[error("Rodio play error: {0}")]
    PlayError(#[from] rodio::PlayError),
    #[error("Rodio stream error: {0}")]
    StreamError(#[from] rodio::StreamError),
    #[error("Cannot get audio devices: {0}")]
    DevicesError(#[from] cpal::DevicesError),
}

struct RodioSink<F> {
    rodio_sink: rodio::Sink,
    _format: F,
    _stream: rodio::OutputStream,
}

fn create_sink(
    host: &cpal::Host,
    device: Option<String>,
) -> Result<(rodio::Sink, rodio::OutputStream), RodioError> {
    let rodio_device = match device {
        Some(device_name) => {
            host.output_devices()?
                .find(|d| d.name().ok().map_or(false, |name| name == device_name)) // Ignore devices for which getting name fails
                .ok_or(RodioError::DeviceNotAvailable(device_name))?
        }
        None => host
            .default_output_device()
            .ok_or(RodioError::NoDeviceAvailable)?,
    };

    let (stream, handle) = rodio::OutputStream::try_from_device(&rodio_device)?;
    let sink = rodio::Sink::try_new(&handle)?;
    Ok((sink, stream))
}

impl<F: SupportedFormat> RodioSink<F>
where
    F::Sample: rodio::Sample,
{
    pub fn open(host: cpal::Host, device: Option<String>) -> Self {
        let (sink, stream) = create_sink(&host, device).unwrap();

        RodioSink {
            rodio_sink: sink,
            _format: F::default(),
            _stream: stream,
        }
    }
}
impl<F: SupportedFormat> Sink for RodioSink<F>
where
    F::Sample: rodio::Sample,
{
    type Format = F;

    fn write(&mut self, samples: &mut [F::Sample]) -> Result<(), SinkError> {
        let buffer = rodio::buffer::SamplesBuffer::new(2, 44100, samples);
        self.rodio_sink.append(buffer);

        // Chunk sizes seem to be about 256 to 3000 ish items long.
        // Assuming they're on average 1628 then a half second buffer is:
        // 44100 elements --> about 27 chunks
        while self.rodio_sink.len() > 26 {
            // sleep and wait for rodio to drain a bit
            thread::sleep(std::time::Duration::from_millis(10));
        }
        Ok(())
    }
}
