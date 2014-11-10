#![feature(phase)]
#[phase(plugin, link)]

extern crate rustradio;
extern crate num;
extern crate portaudio;

use num::Complex;
use portaudio::{pa, types};

use rustradio::file;
use rustradio::blocks::modem::PhaseDiffs;
use rustradio::blocks::RadioBlock;
use rustradio::blocks::filter::{RationalResampler,
                                low_pass_filter_taps,
                                HammingWindow,
                                NumTaps};

fn main() {
    let input_filename = Path::new("/home/allen/repos/gr-tutorial/examples/tutorial6/fm_101.8MHz_1Msps.cfile");
    let audio_sample_rate = 50000.0;
    let pa_buffer_size = 2048;

    /*
        Set up processing blocks
    */
    let lp_taps = low_pass_filter_taps(HammingWindow, 100e3 / 2e6, NumTaps(100));
    // Until we have multidispatch, the samples and the taps need to be the same type
    let lp_taps: Vec<Complex<f32>> = lp_taps.into_iter()
                                            .map(|x| Complex{re: x, im: 0.0}).collect();
    let rf_resampler = RationalResampler{up: 1, down: 2, taps: lp_taps.as_slice()};

    let lp_taps = low_pass_filter_taps(HammingWindow, 1.0/20.0, NumTaps(100));
    let audio_resampler = RationalResampler{up: 1, down: 10, taps: lp_taps.as_slice()};

    /*
        Connect blocks
    */
    let source = file::read_interleaved_float(&input_filename);
    connect!(downsampled <- rf_resampler (source));
    connect!(diffs <- PhaseDiffs (downsampled));
    connect!(mut audio <- audio_resampler (diffs));

    /*
        Set up PortAudio output
    */
    pa::initialize();
    let mut stream : pa::PaStream<f32> = pa::PaStream::new(types::PaFloat32);
    stream.open_default(audio_sample_rate, 0, 0, 1, types::PaFloat32);
    stream.start();

    let mut samples: Vec<f32> = Vec::with_capacity(pa_buffer_size);
    for sample in audio {
        samples.push(sample);
        if samples.len() == pa_buffer_size {
            stream.write(samples.clone(), pa_buffer_size as u32);
            samples.clear();
        }
    }

    stream.close();
    pa::terminate();
}
