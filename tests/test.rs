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
#[allow(unused_variable)]
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
    let taps = vec![1i, -1, 1];
    let b_filter = FilterFIR{ taps: taps.as_slice() };
    connect!(filtered <- b_filter (source));
    let collected: Vec<int> = filtered.take(6).collect();

    assert_eq!(collected, vec![0i, 1, 1, 2, 3, 4]);
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
