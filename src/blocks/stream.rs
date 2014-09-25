//! These blocks are for processing and manipulating streams of (almost) any type.

use std::iter;
use std::option;

use super::{RadioBlock, Hack};
use super::itertools;

pub struct Identity;
impl<A, I: Iterator<A>> RadioBlock<A, A, I, I, ()> for Hack<Identity> {
    fn process(&self, input: I, _: ()) -> I {
        input
    }
}

/// Splits a stream into two identical streams
pub struct Split;
impl<'a, A: Clone, I: Iterator<A>> RadioBlock<A, (A, A), I, iter::Map<'a, A, (A, A), I>, ()> for Hack<Split> {
    fn process(&self, input: I, _: ()) -> iter::Map<'a, A, (A, A), I> {
        input.map(|x| (x.clone(), x))
    }
}

/// Interleaves two streams into one stream.
pub struct Interleave;
pub type InterleaveOutput<'a, A, I> = iter::FlatMap<'a,(A,A),I,iter::Chain<option::Item<A>,option::Item<A>>>;
impl<'a, A: Clone, I: Iterator<(A, A)>> RadioBlock<(A, A), A, I, InterleaveOutput<'a, A, I>, ()> for Hack<Interleave> {
    fn process(&self, input: I, _: ()) -> InterleaveOutput<'a, A, I> {
        input.flat_map(|(l, r)| Some(l).into_iter().chain(Some(r).into_iter()))
    }
}

pub struct DeInterleave;
impl<'a, A, I: Iterator<A>> RadioBlock<A, (A,A), I, itertools::MapChunk2<'a, A, (A,A), I>, ()> for Hack<DeInterleave> {
    fn process(&self, input: I, _: ()) -> itertools::MapChunk2<'a, A, (A,A), I> {
        itertools::map_chunk_2(input, |[x,y]| (x,y))
    }
}

/// Multiplies two streams.
pub struct Multiply;
impl<'a, B, C, A: Mul<B,C>, I: Iterator<(A,B)>> RadioBlock<(A,B), C, I, iter::Map<'a, (A,B), C, I>, ()> for Hack<Multiply> {
    fn process(&self, input: I, _: ()) -> iter::Map<'a, (A,B), C, I> {
        input.map(|(a,b)| a * b)
    }
}

/// Takes every `n`th element.
pub struct Stride;
pub struct Strider<I> {
    iterator: I,
    stride: uint
}
impl<A, I: Iterator<A>> Iterator<A> for Strider<I> {
    fn next(&mut self) -> Option<A> {
        self.iterator.nth(self.stride)
    }
}
impl<A, I: Iterator<A>> RadioBlock<A, A, I, Strider<I>, uint> for Hack<Stride> {
    fn process(&self, input: I, stride: uint) -> Strider<I> {
        Strider{ iterator: input, stride: stride }
    }
}