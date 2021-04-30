// Reads F32 samples from stdin and outputs them using rodio.

use std::io::Read;

use sinky::{filters, formats, Sink, SinkMaker};

struct ExampleSinkMaker;

impl SinkMaker for ExampleSinkMaker {
    type Output = Box<dyn Sink<Format = formats::F32>>;

    fn apply_filters<S: Sink + 'static>(&self, sink: S) -> Self::Output {
        use filters::SinkExt;

        sink.add_converter(filters::SampleConverter::default())
            .add_filter(filters::Volume(0.8))
            .boxed()
    }
}

fn main() {
    const BUFFER_SIZE: usize = 1024;

    let mut backend = sinky::backends::open_rodio(sinky::AudioFormatEnum::I16, ExampleSinkMaker);

    let mut stdin = std::io::stdin();
    let mut buf = [0_u8; BUFFER_SIZE];

    backend.start().unwrap();

    while stdin.read_exact(&mut buf).is_ok() {
        let samples: &mut [f32] =
            unsafe { std::slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut _, BUFFER_SIZE / 4) };

        backend.write(samples).unwrap();
    }

    backend.stop().unwrap();
}
