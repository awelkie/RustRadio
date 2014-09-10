//! These blocks are for processing and manipulating streams of (almost) any type.

use std::iter;
use std::option;

use super::{RadioBlock, Hack};

pub struct Identity;
impl<'a, A: Clone, I: Iterator<A>> RadioBlock<A, A, I, iter::Map<'a, A, A, I>, ()> for Hack<Identity> {
    fn process(&self, input: I, _: ()) -> iter::Map<'a, A, A, I> {
        input.map(|x| x.clone() )
    }
}

/// Splits a stream into two identical streams
pub struct Split;
impl<'a, A: Clone, I: Iterator<A>> RadioBlock<A, (A, A), I, iter::Map<'a, A, (A, A), I>, ()> for Hack<Split> {
    fn process(&self, input: I, _: ()) -> iter::Map<'a, A, (A, A), I> {
        input.map(|x| (x.clone(), x.clone()))
    }
}

/// Interleaves two streams into one stream.
pub struct Interleave;
type InterleaveOutput<'a, A, I> = iter::FlatMap<'a,(A,A),I,iter::Chain<option::Item<A>,option::Item<A>>>;
#[allow(visible_private_types)]
impl<'a, A: Clone, I: Iterator<(A, A)>> RadioBlock<(A, A), A, I, InterleaveOutput<'a, A, I>, ()> for Hack<Interleave> {
    fn process(&self, input: I, _: ()) -> iter::FlatMap<'a,(A,A),I,iter::Chain<option::Item<A>,option::Item<A>>> {
        input.flat_map(|(l, r)| Some(l.clone()).move_iter().chain(Some(r.clone()).move_iter()))
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
struct Strider<I> {
    iterator: I,
    stride: uint
}
impl<A, I: Iterator<A>> Iterator<A> for Strider<I> {
    fn next(&mut self) -> Option<A> {
        self.iterator.nth(self.stride)
    }
}
#[allow(visible_private_types)]
impl<A, I: Iterator<A>> RadioBlock<A, A, I, Strider<I>, uint> for Hack<Stride> {
    fn process(&self, input: I, stride: uint) -> Strider<I> {
        Strider{ iterator: input, stride: stride }
    }
}
