#![feature(phase)]
#![feature(globs)]

#[phase(plugin, link)]
extern crate rustradio;

use std::iter;

//Note: This is needed for the macro to work. TODO: fix this
#[allow(unused_imports)]
use rustradio::blocks::RadioBlock;

#[test]
fn split() {
    let source = vec![0u, 1, 2, 3, 4].move_iter();

    connect!(b1 <- rustradio::blocks::Identity () {source});
    connect!((b2a, b2b) <- rustradio::blocks::Split () {b1});
    connect!(mut b3 <- rustradio::blocks::Multiply () {(b2a, b2b)});

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
    connect!((block_a, block_b) <- rustradio::blocks::Split () {source});
    connect!(sparse <- rustradio::blocks::Stride (100) {block_a});
    connect!(together <- rustradio::blocks::Interleave () {(sparse, block_b)});
    
    let collected: Vec<uint> = together.take(1000000).collect();
}

#[test]
fn filter_fir() {
    let source = iter::count(0i,1);
    let filter = vec![1i, -1, 1];
    connect!(filtered <- rustradio::blocks::FilterFIR (filter.as_slice()) {source});
    let collected: Vec<int> = filtered.take(6).collect();

    assert_eq!(collected, vec![0i, 1, 1, 2, 3, 4]);
}
