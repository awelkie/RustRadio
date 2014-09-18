#![feature(phase)]
#![feature(globs)]

#[phase(plugin, link)]
extern crate rustradio;

extern crate num;
use std::iter;

//Note: This is needed for the macro to work. TODO: fix this
#[allow(unused_imports)]
use rustradio::blocks::RadioBlock;

use rustradio::blocks::stream::*;
use rustradio::blocks::filter::*;
use rustradio::blocks::modem::*;

#[test]
fn split() {
    let source = vec![0u, 1, 2, 3, 4].into_iter();

    connect!(b1 <- Identity () {source});
    connect!((b2a, b2b) <- Split () {b1});
    connect!(mut b3 <- Multiply () {(b2a, b2b)});

    let collected: Vec<uint> = b3.collect();
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
    connect!((block_a, block_b) <- Split () {source});
    connect!(sparse <- Stride (100) {block_a});
    connect!(together <- Interleave () {(sparse, block_b)});
    
    let collected: Vec<uint> = together.take(1000000).collect();
}

#[test]
fn filter_fir() {
    let source = iter::count(0i,1);
    let filter = vec![1i, -1, 1];
    connect!(filtered <- FilterFIR (filter.as_slice()) {source});
    let collected: Vec<int> = filtered.take(6).collect();

    assert_eq!(collected, vec![0i, 1, 1, 2, 3, 4]);
}

#[test]
fn phase_differences() {
    let phase_diffs = vec![0.3f32, 0.2, -2f32, 0f32];
    let source = phase_diffs.iter().map(|&x| x);
    connect!(samples <- FreqMod () {source});
    connect!(diffs <- PhaseDiffs () {samples});

    // assert that they're close enough
    let sse = phase_diffs.iter().zip(diffs).fold(0f32, |sse, (&b,c)| sse + (c - b) * (c - b));
    assert!(sse < 0.001f32);
}
