extern crate portaudio;

use portaudio::{pa, types};

/// Plays constant A4
#[allow(unreachable_code)]
fn main() {
    let sample_rate = 32000.0;
    let sine_freq = 440u;
    let buffer_size = 1024;

    pa::initialize();

    let mut stream : pa::PaStream<f32> = pa::PaStream::new(types::PaFloat32);
    stream.open_default(sample_rate, 0, 0, 1, types::PaFloat32);
    stream.start();

    let mut phase = 0f32;
    let radians_per_sample = (sine_freq as f32) / (sample_rate as f32) * Float::two_pi();
    loop {
        let samples: Vec<f32> = Vec::from_fn(buffer_size as uint, |_| {
            phase += radians_per_sample;
            phase.sin()
        });
        stream.write(samples, buffer_size);
    }

    stream.close();
    pa::terminate();
}
