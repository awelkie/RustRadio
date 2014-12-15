//! These blocks are for processing and manipulating streams of (almost) any type.

use std::iter::{Map, Chain, FlatMap};
use std::option::{Item};

use super::RadioBlock;
use IteratorExtras::{IteratorExtra, MapPairs};
use IteratorExtras;

/// Splits a stream into two identical streams
#[deriving(Copy)]
pub struct Split;
impl<'a, A, I> RadioBlock<A, (A, A), I, Map<'a, A, (A, A), I>> for Split 
where A: Clone, I: Iterator<A> {
    fn process(&self, input: I) -> Map<'a, A, (A, A), I> {
        input.map(|x| (x.clone(), x))
    }
}

/// Interleaves two streams into one stream.
#[deriving(Copy)]
pub struct Interleave;
impl<'a, A, I> RadioBlock<(A, A), A, I, FlatMap<'a,(A,A),I,Chain<Item<A>,Item<A>>>> for Interleave
where A: Clone, I: Iterator<(A, A)> {
    fn process(&self, input: I) -> FlatMap<'a,(A,A),I,Chain<Item<A>,Item<A>>> {
        input.flat_map(|(l, r)| Some(l).into_iter().chain(Some(r).into_iter()))
    }
}

#[deriving(Copy)]
pub struct DeInterleave;
impl<'a, A, I> RadioBlock<A, (A,A), I, MapPairs<'a, A, (A,A), I>> for DeInterleave
where I: Iterator<A> {
    fn process(&self, input: I) -> MapPairs<'a, A, (A,A), I> {
        input.map_pairs(|[x,y]| (x,y))
    }
}

/// Multiplies two streams.
#[deriving(Copy)]
pub struct Multiply;
impl<'a, A, B, C, I> RadioBlock<(A,B), C, I, Map<'a, (A,B), C, I>> for Multiply
where A: Mul<B,C>, I: Iterator<(A,B)> {
    fn process(&self, input: I) -> Map<'a, (A,B), C, I> {
        input.map(|(a,b)| a * b)
    }
}

/// Takes every `n`th element.
#[deriving(Copy)]
pub struct Stride {
    pub stride: uint,
}
impl<A, I> RadioBlock<A, A, I, IteratorExtras::Stride<A, I>> for Stride
where I: Iterator<A> {
    fn process(&self, input: I) -> IteratorExtras::Stride<A, I> {
        input.stride(self.stride)
    }
}
