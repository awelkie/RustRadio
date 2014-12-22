#![feature(phase)]
#[phase(plugin, link)]

extern crate rustradio;
extern crate rtlsdr;
extern crate num;

use num::Complex;
use std::io;

use rtlsdr::RTLSDR;
use rustradio::file::write_stream;
use rustradio::blocks::modem::PhaseDiffs;
use rustradio::blocks::RadioBlock;
use rustradio::blocks::filter::NumTapsSpecifier::NumTaps;
use rustradio::blocks::filter::{RationalResampler,
                                low_pass_filter_taps,
                                HammingWindow};

fn main() {
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
        Set up RTL-SDR source
    */
    let mut source = RTLSDR::new().unwrap();
    if let Err(_) = source.set_sample_rate(1_000_000) {
        panic!("Error setting sample rate");
    }
    if let Err(_) = source.set_freq(101_100_000) {
        panic!("Error setting frequency");
    }

    /*
        Connect blocks
    */
    connect!(downsampled <- rf_resampler (source));
    connect!(diffs <- PhaseDiffs (downsampled));
    connect!(audio <- audio_resampler (diffs));

    // to play the audio, you can pipe stdout to an audio player. For example, if you have `sox`
    // installed, you can run:
    // ./target/release/fm_demod | play -r 50k -t raw -e float -b 32 -c 1 -V1 -
    write_stream(io::stdout(), audio);
}
