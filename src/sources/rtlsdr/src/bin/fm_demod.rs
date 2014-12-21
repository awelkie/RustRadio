#![feature(phase)]
#[phase(plugin, link)]

extern crate rustradio;
extern crate rtlsdr;
extern crate num;

use num::Complex;
use std::io::stdio::stdout;

use rtlsdr::RTLSDR;
use rustradio::file::{file_read_stream, write_stream};
use rustradio::blocks::modem::PhaseDiffs;
use rustradio::blocks::RadioBlock;
use rustradio::blocks::filter::{RationalResampler,
                                low_pass_filter_taps,
                                HammingWindow,
                                NumTapsSpecifier};

fn main() {
    //let input_filename = Path::new("/home/allen/repos/gr-tutorial/examples/tutorial6/fm_101.8MHz_1Msps.cfile");

    /*
        Set up processing blocks
    */
    let lp_taps = low_pass_filter_taps(HammingWindow, 100e3 / 2e6, NumTapsSpecifier::NumTaps(100));
    // Until we have multidispatch, the samples and the taps need to be the same type
    let lp_taps: Vec<Complex<f32>> = lp_taps.into_iter()
                                            .map(|x| Complex{re: x, im: 0.0}).collect();
    let rf_resampler = RationalResampler{up: 1, down: 2, taps: lp_taps.as_slice()};

    let lp_taps = low_pass_filter_taps(HammingWindow, 1.0/20.0, NumTapsSpecifier::NumTaps(100));
    let audio_resampler = RationalResampler{up: 1, down: 10, taps: lp_taps.as_slice()};

    /*
        Connect blocks
    */
    //let source = file_read_stream::<Complex<f32>>(&input_filename);
    let mut source = RTLSDR::new().unwrap();
    source.set_sample_rate(1_000_000);
    source.set_freq(101_100_000);
    connect!(downsampled <- rf_resampler (source));
    connect!(diffs <- PhaseDiffs (downsampled));
    connect!(audio <- audio_resampler (diffs));
    write_stream(stdout(), audio);
}
