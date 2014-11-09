#![feature(phase)]
#[phase(plugin, link)]

extern crate rustradio;
extern crate num;

use num::Complex;

use rustradio::file;
use rustradio::blocks::modem::PhaseDiffs;
use rustradio::blocks::RadioBlock;
use rustradio::blocks::filter::{RationalResampler,
                                low_pass_filter_taps,
                                HammingWindow,
                                NumTaps};

fn main() {
    let input_filename = Path::new("/home/allen/repos/gr-tutorial/examples/tutorial6/fm_101.8MHz_1Msps.cfile");
    let output_filename = Path::new("./output.bin");

    let lp_taps = low_pass_filter_taps(HammingWindow, 100e3 / 4e6, NumTaps(100));
    // Until we have multidispatch, the samples and the taps need to be the same type
    let lp_taps: Vec<Complex<f32>> = lp_taps.into_iter()
                                            .map(|x| Complex{re: x, im: 0.0}).collect();
    let Resampler1 = RationalResampler{up: 1, down: 2, taps: lp_taps.as_slice()};

    let lp_taps = low_pass_filter_taps(HammingWindow, 1.0/20.0, NumTaps(100));
    let Resampler2 = RationalResampler{up: 1, down: 10, taps: lp_taps.as_slice()};

    let mut s_count = 0u;
    let mut r1_count = 0u;
    let mut pd_count = 0u;
    let mut r2_count = 0u;

    {
        let source = file::read_interleaved_float(&input_filename);
        let source = source.inspect(|_| s_count+=1);
        connect!(downsampled <- Resampler1 (source));
        let downsampled = downsampled.inspect(|_| r1_count+=1);
        connect!(diffs <- PhaseDiffs (downsampled));
        let diffs = diffs.inspect(|_| pd_count+=1);
        connect!(audio <- Resampler2 (diffs));
        let audio = audio.inspect(|_| r2_count+=1);
        file::write_float(&output_filename, audio);
    }

    println!("{}, {}, {}, {}", s_count, r1_count, pd_count, r2_count);
}
