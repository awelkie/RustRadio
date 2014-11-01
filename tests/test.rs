#![feature(phase)]
#![feature(globs)]
#[allow(unused_imports)]
#[phase(plugin, link)]

extern crate rustradio;
extern crate num;

//Note: This is needed for the macro to work. TODO: fix this
use rustradio::blocks::RadioBlock;
use std::iter;

use rustradio::blocks::stream::*;
use rustradio::blocks::filter::*;
use rustradio::blocks::modem::*;

#[test]
fn split() {
    let source = vec![0u, 1, 2, 3, 4].into_iter();

    connect!((s1, s2) <- Split (source));
    connect!(mut combined <- Multiply (s1, s2));

    let collected: Vec<uint> = combined.collect();
    assert_eq!(collected, vec![0u, 1, 4, 9, 16]);
}

#[test]
#[should_fail]
#[allow(unused_variables)]
fn split_buffer_overrun() {
    /*
        This should fail because the flowgraph requires unbounded memory growth, which
        fixed buffers can't satisfy. Note that an equivalent graph locks up in GNURadio
    */
    let source = iter::count(0u,1);

    let b_stride = Stride { stride: 100 };

    connect!((block_a, block_b) <- Split (source));
    connect!(sparse <- b_stride (block_a));
    connect!(together <- Interleave (sparse, block_b));

    let collected: Vec<uint> = together.take(1000000).collect();
}

#[test]
fn filter_fir() {
    let source = iter::count(0i,1);
    let taps = vec![1i, 2, 3];
    let b_filter = FilterFIR{ taps: taps.as_slice() };
    connect!(filtered <- b_filter (source));
    let collected: Vec<int> = filtered.take(6).collect();

    assert_eq!(collected, vec![0i, 1, 4, 10, 16, 22]);
}

#[test]
fn phase_differences() {
    let phase_diffs = vec![0.3f32, 0.2, -2f32, 0f32];
    let source = phase_diffs.iter().map(|&x| x);

    connect!(samples <- FreqMod (source));
    connect!(diffs <- PhaseDiffs (samples));

    // assert that they're close enough
    let sse = phase_diffs.iter().zip(diffs).fold(0f32, |sse, (&b,c)| sse + (c - b) * (c - b));
    assert!(sse < 0.001f32);
}

#[test]
// This tests that, at upsample = downsample = 1, the RationalResampler
// is just an FIR filter.
fn resampler_is_filter() {
    let taps = vec![1i, -1i, 2, 3];
    let source = iter::count(0i, 1).take(10);
    let source_copy = source.clone();

    let b_filter = FilterFIR{ taps: taps.as_slice() };
    let b_resampler = RationalResampler{ up: 1, down: 1, taps: taps.as_slice() };

    connect!(mut fir_filtered <- b_filter (source_copy));
    connect!(mut resampler_filtered <- b_resampler (source));

    let fir_filtered: Vec<int> = fir_filtered.collect();
    let resampler_filtered: Vec<int> = resampler_filtered.collect();
    assert_eq!(fir_filtered, resampler_filtered);
}

#[test]
// Tests a couple of known rational resampler outputs
fn test_resampler() {

    let source = iter::count(0i, 1).take(5);
    let taps = vec![1i, -1, 1];
    let b_resampler = RationalResampler{ up: 2, down: 1, taps: taps.as_slice() };
    connect!(mut resampled <- b_resampler (source));
    let resampled: Vec<int> = resampled.collect();
    assert_eq!(resampled, vec![0i, 0, 1, -1, 3, -2, 5, -3, 7, -4]);

    let source = iter::count(0i, 1).take(5);
    let taps = vec![1i];
    let b_resampler = RationalResampler{ up: 2, down: 1, taps: taps.as_slice() };
    connect!(mut resampled <- b_resampler (source));
    let resampled: Vec<int> = resampled.collect();
    assert_eq!(resampled, vec![0i, 0, 1, 0, 2, 0, 3, 0, 4, 0]);

    let source = iter::count(0i, 1).take(10);
    let taps = vec![1i];
    let b_resampler = RationalResampler{ up: 1, down: 2, taps: taps.as_slice() };
    connect!(mut resampled <- b_resampler (source));
    let resampled: Vec<int> = resampled.collect();
    assert_eq!(resampled, vec![0i, 2, 4, 6, 8]);

    let source = iter::count(0i, 1).take(5);
    let taps = vec![1i, -1];
    let b_resampler = RationalResampler{ up: 2, down: 1, taps: taps.as_slice() };
    connect!(mut resampled <- b_resampler (source));
    let resampled: Vec<int> = resampled.collect();
    assert_eq!(resampled, vec![0, 0, 1, -1, 2, -2, 3, -3, 4, -4]);

    let source = iter::count(0i, 1).take(5);
    let taps = vec![1i, -1, 1];
    let b_resampler = RationalResampler{ up: 3, down: 5, taps: taps.as_slice() };
    connect!(mut resampled <- b_resampler (source));
    let resampled: Vec<int> = resampled.collect();
    assert_eq!(resampled, vec![0, 1, -3]);

}
