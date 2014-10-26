extern crate pasimple;

/// Plays constant A4
fn main() {
    let sample_rate = 32000;
    let sine_freq = 440u;

    let (tx, rx) = channel();
    spawn(proc() {
        pasimple::pulse_sink(rx, sample_rate);
    });

    let mut phase = 0f32;
    let radians_per_sample = (sine_freq as f32) / (sample_rate as f32) * Float::two_pi();
    let samples_per_chunk = 10000;
    loop {
        let samples: Vec<f32> = Vec::from_fn(samples_per_chunk, |_| {
            phase += radians_per_sample;
            phase.sin()
        });
        tx.send(samples);
    }
}
